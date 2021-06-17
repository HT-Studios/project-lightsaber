use x86_64::{
    registers::control::Cr3,
    structures::paging::{
        mapper::MapToError,
        FrameAllocator,
        Mapper,
        OffsetPageTable,
        Page,
        PageTable,
        PageTableFlags,
        PhysFrame,
        Size4KiB
    },
    PhysAddr,
    VirtAddr
};

use lightsaber_bootloader::{
    MemoryRegionType,
    MemoryRegions
};

pub struct GlobalAllocator {
    memory_map: &'static MemoryRegions,
    next: usize
}

impl GlobalAllocator {
    pub unsafe fn initialize(memory_map: &'static MemoryRegions) -> Self {
        Self {
            memory_map,
            next: 0
        }
    }

    pub fn frame_type(&self, frame: PhysFrame) -> Option<MemoryRegionType> {
        self.memory_map
            .iter()
            .find(|region| {
                let address = frame.start_address().as_u64();
                region.start >= address && address < region.end
            })
            .map(|region| region.region_type)
    }

    pub(in self) fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|region| region.region_type == MemoryRegionType::Usable);
        let address_ranges = usable_regions.map(|region| region.start..region.end);
        let frame_addresses = address_ranges.flat_map(|region| region.step_by(4096));

        frame_addresses.map(|address| PhysFrame::containing_address(PhysAddr::new(address)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for GlobalAllocator {
    #[track_caller]
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;

        frame
    }
}

unsafe impl Send for GlobalAllocator { }
unsafe impl Sync for GlobalAllocator { }

pub struct UnmapGuard {
    pub page: Page<Size4KiB>
}

impl UnmapGuard {
    #[inline]
    pub(in self) fn new(page: Page<Size4KiB>) -> Self {
        Self {
            page
        }
    }
}

pub fn initialize_paging(physical_memory_offset: VirtAddr, memory_regions: &'static MemoryRegions) -> (OffsetPageTable<'static>, GlobalAllocator) {
    unsafe {
        let active_level_four = active_level_four_table(physical_memory_offset);

        let offset_page_table = OffsetPageTable::new(active_level_four, physical_memory_offset);
        let frame_allocator = GlobalAllocator::initialize(memory_regions);

        (offset_page_table, frame_allocator)
    }
}

pub unsafe fn active_level_four_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let (level_four_table_frame, _) = Cr3::read();
    let physical = level_four_table_frame.start_address();
    let virtual_address = physical_memory_offset + physical.as_u64();
    let page_table_ptr: *mut PageTable = virtual_address.as_mut_ptr();

    &mut *page_table_ptr
}

pub unsafe fn memory_map_device(offset_table: &mut OffsetPageTable, frame_allocator: &mut GlobalAllocator, frame: PhysFrame) -> Result<UnmapGuard, MapToError<Size4KiB>> {
    let frame_type = frame_allocator
        .frame_type(frame)
        .ok_or(MapToError::FrameAllocationFailed)?;

    let extra_flags = match frame_type {
        MemoryRegionType::UnknownBios(_) | MemoryRegionType::UnknownUefi(_) => {
            PageTableFlags::WRITABLE
        }
        _ => panic!(
            "Attmpted to memory map a device on a {:?} frame {:#?}",
            frame_type,
            frame.start_address()
        )
    };

    let page = Page::containing_address(VirtAddr::new(frame.start_address().as_u64()));

    offset_table
        .identity_map(
            frame,
            PageTableFlags::PRESENT
                | PageTableFlags::NO_CACHE
                | PageTableFlags::WRITE_THROUGH
                | extra_flags,
            frame_allocator
        )?
        .flush();

    Ok(UnmapGuard::new(page))
}
