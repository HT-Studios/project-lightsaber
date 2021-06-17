use core::{
    mem::{
        self,
        MaybeUninit
    },
    slice
};

use uefi::{
    proto::media::{
        file::{
            File,
            FileAttribute,
            FileMode,
            FileInfo,
            FileSystemVolumeLabel,
            RegularFile
        },
        fs::SimpleFileSystem
    },
    prelude::{
        BootServices,
        ResultExt
    },
    table::boot::{
        AllocateType,
        MemoryType
    }
};

use xmas_elf::{
    header,
    program::{
        self,
        ProgramHeader
    },
    ElfFile
};

use x86_64::{
    registers,
    structures::paging::{
        FrameAllocator,
        Mapper,
        OffsetPageTable,
        Page,
        PageSize,
        PageTableFlags,
        PageTableIndex,
        PhysFrame,
        Size4KiB,
        Size2MiB
    },
    align_up,
    PhysAddr,
    VirtAddr
};

use lightsaber_bootloader::{
    BootInformation,
    MemoryRegion,
    UnwindInformation
};

use lightsaber_graphics::{
    Framebuffer,
    FramebufferInformation
};

use crate::paging::{
    self,
    BootFrameAllocator,
    BootMemoryRegion,
    PageTables,
    ReservedFrames
};

const SIZE_4_KIB_ZERO_ARRAY: Size4KiBPageArray = [0; Size4KiB::SIZE as usize / 8];
type Size4KiBPageArray = [u64; Size4KiB::SIZE as usize / 8];

#[derive(Debug)]
pub struct LevelFourEntries {
    entries: [bool; 512]
}

impl LevelFourEntries {
    pub(in self) fn new<'a>(segments: impl Iterator<Item = ProgramHeader<'a>>) -> Self {
        let mut rawself = Self {
            entries: [false; 512]
        };

        rawself.entries[0] = true;

        segments.for_each(|segment| {
            let start_page: Page = Page::containing_address(VirtAddr::new(segment.virtual_addr()));
            let end_page: Page = Page::containing_address(VirtAddr::new(segment.virtual_addr() + segment.mem_size()));

            (u64::from(start_page.p4_index())..=u64::from(end_page.p4_index())).for_each(|p4_index| {
                rawself.entries[p4_index as usize] = true;
            });
        });

        rawself
    }

    pub(in self) fn get_free_entry(&mut self) -> PageTableIndex {
        let (index, entry) = self
            .entries
            .iter_mut()
            .enumerate()
            .find(|(_, &mut entry)| !entry)
            .expect("No available Level Four entries.");

        *entry = true;
        PageTableIndex::new(index as u16)
    }

    pub(in self) fn get_free_address(&mut self) -> VirtAddr {
        Page::from_page_table_indices_1gib(self.get_free_entry(), PageTableIndex::new(0))
            .start_address()
    }
}

pub struct Mappings {
    pub entry_point: VirtAddr,
    pub stack_end: Page,
    pub used_entries: LevelFourEntries,
    pub framebuffer: VirtAddr,
    pub physical_memory_offset: VirtAddr
}

#[derive(Clone, Copy, Debug)]
pub struct SystemInformation {
    pub framebuffer_address: PhysAddr,
    pub framebuffer_information: FramebufferInformation,
    pub rsdp_address: PhysAddr
}

pub fn load_and_switch_to_kernel<I, D>(mut frame_allocator: BootFrameAllocator<I, D>, mut page_tables: PageTables, kernel_bytes: &[u8], system_info: SystemInformation, ) -> !
    where
        I: ExactSizeIterator<Item = D> + Clone,
        I::Item: BootMemoryRegion,
{
    let (kernel_entry, used_entries) =
        load_kernel(&mut frame_allocator, &mut page_tables, kernel_bytes);

    let mut mappings = setup_mappings(
        &mut frame_allocator,
        &mut page_tables,
        system_info,
        kernel_entry,
        used_entries,
    );

    let unwind_information = UnwindInformation {
        kernel_base: VirtAddr::new(kernel_bytes[0] as u64),
        kernel_size: kernel_bytes.len(),
        stack_top: mappings.stack_end.start_address()
    };

    let (boot_information, mut reserved_frames) = create_boot_information(
        frame_allocator,
        &mut page_tables,
        &mut mappings,
        system_info,
        unwind_information
    );

    log::info!(
        "Jumping to System Kernel entry point at {:?}.",
        mappings.entry_point
    );

    let current_addr = PhysAddr::new(registers::read_rip().as_u64());
    let current_frame: PhysFrame = PhysFrame::containing_address(current_addr);

    PhysFrame::range_inclusive(current_frame, current_frame + 1).for_each(|frame| {
        unsafe {
            page_tables.kernel_page_table.identity_map(
                frame,
                PageTableFlags::PRESENT,
                &mut reserved_frames,
            )
                .unwrap()
                .flush();
        }
    });

    mem::drop(page_tables.kernel_page_table);

    unsafe {
        let kernel_level_four_start = page_tables.kernel_level_four_frame.start_address().as_u64();
        let stack_top = mappings.stack_end.start_address().as_u64();
        let entry_point = mappings.entry_point.as_u64();

        asm!(
        "mov cr3, {}; mov rsp, {}; push 0; jmp {}",
        in(reg) kernel_level_four_start,
        in(reg) stack_top,
        in(reg) entry_point,
        in("rdi") boot_information as *const _ as usize,
        )
    };

    unreachable!()
}

pub fn load_file(boot_services: &BootServices, path: &str) -> &'static [u8] {
    let mut information_buffer = [0u8; 0x100];

    let filesystem = unsafe {
        &mut *boot_services
            .locate_protocol::<SimpleFileSystem>()
            .expect_success("Failed to locate simple fileststem.")
            .get()
    };

    let mut root = filesystem
        .open_volume()
        .expect_success("Failed to open root volume.");

    let volume_label = filesystem
        .open_volume()
        .expect_success("Failed to open filesystem volume.")
        .get_info::<FileSystemVolumeLabel>(&mut information_buffer)
        .expect_success("Failed to retrieve filesystem volume label.")
        .volume_label();

    log::info!("Found filesystem volume label: {}", volume_label);

    let file_handle = root
        .open(path, FileMode::Read, FileAttribute::empty())
        .expect_success("Failed to open file.");
    let mut regular_file = unsafe {
        RegularFile::new(file_handle)
    };

    log::info!("Loading file {} into memory.", path);

    let information = regular_file
        .get_info::<FileInfo>(&mut information_buffer)
        .expect_success("Failed to retrieve file information.");

    let pages = information.file_size() as usize / 0x1000 + 1;
    let memory_start = boot_services
        .allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, pages)
        .expect_success("Failed to allocate pages.");

    let buffer = unsafe {
        slice::from_raw_parts_mut(memory_start as *mut u8, pages * 0x1000)
    };
    let length = regular_file
        .read(buffer)
        .expect_success("Failed to read file.");

    buffer[..length].as_ref()
}

pub(in self) fn create_boot_information<I, D>(mut frame_allocator: BootFrameAllocator<I, D>, page_tables: &mut PageTables, mappings: &mut Mappings, system_information: SystemInformation, unwind_information: UnwindInformation) -> (&'static mut BootInformation, ReservedFrames)
    where
        I: ExactSizeIterator<Item = D> + Clone,
        I::Item: BootMemoryRegion {
    let (boot_information, memory_regions) = {
        let boot_information_address = mappings.used_entries.get_free_address();
        let boot_information_end = boot_information_address + mem::size_of::<BootInformation>();

        let memory_map_regions_address = boot_information_end.align_up(mem::align_of::<MemoryRegion>() as u64);
        let regions = frame_allocator.len() + 1;
        let memory_map_regions_end = memory_map_regions_address + regions * mem::size_of::<MemoryRegion>();

        let start_page = Page::containing_address(boot_information_address);
        let end_page = Page::containing_address(memory_map_regions_end - 1u64);

        Page::range_inclusive(start_page, end_page).for_each(|page| {
            let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
            let frame = frame_allocator.allocate_frame()
                .expect("Failed to allocate frames for boot information.");

            unsafe {
                page_tables
                    .kernel_page_table
                    .map_to(page, frame, flags, &mut frame_allocator)
                    .unwrap()
                    .flush();
            }

            unsafe {
                page_tables
                    .boot_page_table
                    .map_to(page, frame, flags, &mut frame_allocator)
                    .unwrap()
                    .flush();
            }
        });

        let boot_information: &'static mut MaybeUninit<BootInformation> = unsafe {
            &mut *boot_information_address.as_mut_ptr()
        };

        let memory_regions: &'static mut [MaybeUninit<MemoryRegion>] = unsafe {
            slice::from_raw_parts_mut(memory_map_regions_address.as_mut_ptr(), regions)
        };

        (boot_information, memory_regions)
    };

    let reserved_frames = ReservedFrames::new(&mut frame_allocator);

    log::info!("Creating memory map.");
    let memory_regions_ = frame_allocator.construct_memory_map(memory_regions);

    log::info!("Creating boot information.");
    let framebuffer = Framebuffer {
        buffer_start: mappings.framebuffer.as_u64(),
        buffer_len_bytes: system_information.framebuffer_information.len_bytes,
        information: system_information.framebuffer_information
    };

    (
        boot_information.write(BootInformation {
            rsdp_address: system_information.rsdp_address,
            physical_memory_offset: mappings.physical_memory_offset,
            framebuffer,
            memory_regions: memory_regions_.into(),
            unwind_information
        }),
        reserved_frames
    )
}

pub(in self) fn load_kernel(frame_allocator: &mut impl FrameAllocator<Size4KiB>, page_tables: &mut PageTables, kernel_bytes: &[u8]) -> (u64, LevelFourEntries) {
    log::info!("Loading System Kernel.");

    paging::efer_update_no_execute_enable();
    paging::cr0_update_write_protect();

    let kernel_elf = ElfFile::new(&kernel_bytes).expect("The System Kernel file is corrupted.");
    let kernel_offset = PhysAddr::new(&kernel_bytes[0] as *const u8 as u64);

    assert!(kernel_offset.is_aligned(Size4KiB::SIZE));

    header::sanity_check(&kernel_elf).expect("The System Kernel file failed the sanity check.");

    let entry_point = kernel_elf.header.pt2.entry_point();
    log::info!("The System Kernel entry point is found at: {:#x}", entry_point);

    kernel_elf.program_iter().for_each(|header| {
        program::sanity_check(header, &kernel_elf).expect("A program in the System Kernel file failed the sanity check.");

        let header_type = header.get_type().expect("Failed to get header type.");
        if let program::Type::Load = header_type {
            map_segment(
                &header,
                kernel_offset,
                frame_allocator,
                &mut page_tables.kernel_page_table
            );
        }
    });

    let used_entries = LevelFourEntries::new(kernel_elf.program_iter());

    (entry_point, used_entries)
}

pub(in self) fn map_segment(segment: &ProgramHeader, kernel_offset: PhysAddr, frame_allocator: &mut impl FrameAllocator<Size4KiB>, page_table: &mut OffsetPageTable) {
    let physical_address = kernel_offset + segment.offset();
    let start_frame: PhysFrame = PhysFrame::containing_address(physical_address);
    let end_frame: PhysFrame = PhysFrame::containing_address(physical_address + segment.file_size() - 1u64);

    let virtual_start = VirtAddr::new(segment.virtual_addr());
    let start_page: Page = Page::containing_address(virtual_start);

    let flags = segment.flags();
    let mut page_table_flags = PageTableFlags::PRESENT;

    if !flags.is_execute() {
        page_table_flags |= PageTableFlags::NO_EXECUTE;
    }

    if flags.is_write() {
        page_table_flags |= PageTableFlags::WRITABLE;
    }

    PhysFrame::range_inclusive(start_frame, end_frame).for_each(|frame| {
        let offset = frame - start_frame;
        let page = start_page + offset;

        unsafe {
            page_table.map_to(page, frame, page_table_flags, frame_allocator)
                .unwrap()
                .ignore();
        }
    });

    if segment.mem_size() > segment.file_size() {
        let zero_start = virtual_start + segment.file_size();
        let zero_end = virtual_start + segment.mem_size();

        if zero_start.as_u64() & 0xFFF != 0 {
            let original_frame: PhysFrame = PhysFrame::containing_address(physical_address + segment.file_size() - 1u64);
            let new_frame = frame_allocator.allocate_frame().unwrap();

            {
                let new_frame_ptr = new_frame.start_address().as_u64() as *mut Size4KiBPageArray;
                unsafe {
                    new_frame_ptr.write(SIZE_4_KIB_ZERO_ARRAY);
                }
            }

            let original_bytes_ptr = original_frame.start_address().as_u64() as *mut u8;
            let new_bytes_ptr = new_frame.start_address().as_u64() as *mut u8;

            (0..((zero_start.as_u64() & 0xFFF) as isize)).for_each(|offset| {
                unsafe {
                    let original_byte = original_bytes_ptr.offset(offset).read();
                    new_bytes_ptr.offset(offset).write(original_byte);
                }
            });

            let last_page = Page::containing_address(virtual_start + segment.file_size() - 1u64);

            unsafe {
                page_table.unmap(last_page).unwrap().1.ignore();
                page_table
                    .map_to(last_page, new_frame, page_table_flags, frame_allocator)
                    .unwrap()
                    .ignore();
            }
        }

        let start_page: Page = Page::containing_address(VirtAddr::new(align_up(zero_start.as_u64(), Size4KiB::SIZE)));
        let end_page = Page::containing_address(zero_end);

        Page::range_inclusive(start_page, end_page).for_each(|page| {
            let frame = frame_allocator.allocate_frame().unwrap();

            {
                let frame_ptr = frame.start_address().as_u64() as *mut Size4KiBPageArray;
                unsafe {
                    frame_ptr.write(SIZE_4_KIB_ZERO_ARRAY);
                }
            }

            unsafe {
                page_table
                    .map_to(page, frame, page_table_flags, frame_allocator)
                    .unwrap()
                    .ignore();
            }
        });
    }
}

pub(in self) fn setup_mappings<I, D>(frame_allocator: &mut BootFrameAllocator<I, D>, page_tables: &mut PageTables, system_information: SystemInformation, kernel_entry: u64, mut used_entries: LevelFourEntries) -> Mappings
    where
        I: ExactSizeIterator<Item = D> + Clone,
        I::Item: BootMemoryRegion {
    let entry_point = VirtAddr::new(kernel_entry);

    log::info!("Creating a stack for the System Kernel.");

    let stack_start_address = used_entries.get_free_address();
    let stack_start: Page = Page::containing_address(stack_start_address);

    let stack_end_address = stack_start_address + (20 * Size4KiB::SIZE);
    let stack_end: Page = Page::containing_address(stack_end_address);

    Page::range_inclusive(stack_start, stack_end).for_each(|page| {
        let frame = frame_allocator.allocate_frame().expect("Failed a frame allocation when mapping a Kernel stack.");
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        unsafe {
            page_tables
                .kernel_page_table
                .map_to(page, frame, flags, frame_allocator)
                .unwrap()
                .flush();
        }
    });

    log::info!("Mapping framebuffer.");

    let framebuffer_start_frame: PhysFrame = PhysFrame::containing_address(system_information.framebuffer_address);
    let framebuffer_end_frame: PhysFrame = PhysFrame::containing_address(system_information.framebuffer_address + system_information.framebuffer_information.len_bytes - 1u64);

    let framebuffer_start_page: Page = Page::containing_address(used_entries.get_free_address());

    PhysFrame::range_inclusive(framebuffer_start_frame, framebuffer_end_frame).enumerate().for_each(|(index, frame)| {
        let page = framebuffer_start_page + index as u64;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        unsafe {
            page_tables
                .kernel_page_table
                .map_to(page, frame, flags, frame_allocator)
                .unwrap()
                .flush();
        }
    });

    let framebuffer = framebuffer_start_page.start_address();

    let physical_memory_offset = used_entries.get_free_address();

    let start_frame = PhysFrame::containing_address(PhysAddr::new(0));
    let max_physical = frame_allocator.max_physical_address();

    let end_frame: PhysFrame<Size2MiB> = PhysFrame::containing_address(max_physical - 1u64);

    PhysFrame::range_inclusive(start_frame, end_frame).for_each(|frame| {
        let page = Page::containing_address(physical_memory_offset + frame.start_address().as_u64());
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        unsafe {
            page_tables
                .kernel_page_table
                .map_to(page, frame, flags, frame_allocator)
                .unwrap()
                .flush();
        }
    });

    Mappings {
        entry_point,
        stack_end,
        used_entries,
        framebuffer,
        physical_memory_offset
    }
}
