use core::mem;

use lightsaber_util::libcore_ext::io;

pub(in self) mod access_flags;
pub mod access_type;
pub(in self) mod entry_flags;
pub(in self) mod segment_selector;

pub(in self) use access_flags::GlobalDescriptorTableAccessFlags;
pub(in self) use access_type::GlobalDescriptorTableAccessType;
pub(in self) use entry_flags::GlobalDescriptorTableEntryFlags;
pub(in self) use segment_selector::SegmentSelector;

pub(in self) const GLOBAL_DESCRIPTOR_TABLE_ENTRY_COUNT: usize = 10;
pub(in self) const TEMPORARY_GLOBAL_DESCRIPTOR_TABLE_ENTRY_COUNT: usize = 4;
pub(in self) const USER_TCB_OFFSET: usize = 0xB0000000;

pub(in self) static FAULT_STACK: [u8; 256] = [0; 256];

#[thread_local]
pub(in self) static mut GLOBAL_DESCRIPTOR_TABLE: [GlobalDescriptorTableEntry; GLOBAL_DESCRIPTOR_TABLE_ENTRY_COUNT] = [
    GlobalDescriptorTableEntry::NULL,
    GlobalDescriptorTableEntry::new(
        GlobalDescriptorTableAccessFlags::PRESENT
            | GlobalDescriptorTableAccessFlags::RING_0
            | GlobalDescriptorTableAccessFlags::SYSTEM
            | GlobalDescriptorTableAccessFlags::EXECUTABLE
            | GlobalDescriptorTableAccessFlags::PRIVILEGE,
        GlobalDescriptorTableEntryFlags::LONG_MODE
    ),
    GlobalDescriptorTableEntry::new(
        GlobalDescriptorTableAccessFlags::PRESENT
            | GlobalDescriptorTableAccessFlags::RING_0
            | GlobalDescriptorTableAccessFlags::SYSTEM
            | GlobalDescriptorTableAccessFlags::PRIVILEGE,
        GlobalDescriptorTableEntryFlags::LONG_MODE
    ),
    GlobalDescriptorTableEntry::new(
        GlobalDescriptorTableAccessFlags::PRESENT
            | GlobalDescriptorTableAccessFlags::RING_0
            | GlobalDescriptorTableAccessFlags::SYSTEM
            | GlobalDescriptorTableAccessFlags::PRIVILEGE,
        GlobalDescriptorTableEntryFlags::LONG_MODE
    ),
    GlobalDescriptorTableEntry::new(
        GlobalDescriptorTableAccessFlags::PRESENT
            | GlobalDescriptorTableAccessFlags::RING_0
            | GlobalDescriptorTableAccessFlags::SYSTEM
            | GlobalDescriptorTableAccessFlags::EXECUTABLE
            | GlobalDescriptorTableAccessFlags::PRIVILEGE,
        GlobalDescriptorTableEntryFlags::PROTECTED_MODE
    ),
    GlobalDescriptorTableEntry::new(
        GlobalDescriptorTableAccessFlags::PRESENT
            | GlobalDescriptorTableAccessFlags::RING_3
            | GlobalDescriptorTableAccessFlags::SYSTEM
            | GlobalDescriptorTableAccessFlags::PRIVILEGE,
        GlobalDescriptorTableEntryFlags::LONG_MODE
    ),
    GlobalDescriptorTableEntry::new(
        GlobalDescriptorTableAccessFlags::PRESENT
            | GlobalDescriptorTableAccessFlags::RING_3
            | GlobalDescriptorTableAccessFlags::SYSTEM
            | GlobalDescriptorTableAccessFlags::EXECUTABLE
            | GlobalDescriptorTableAccessFlags::PRIVILEGE,
        GlobalDescriptorTableEntryFlags::LONG_MODE
    ),
    GlobalDescriptorTableEntry::new(
        GlobalDescriptorTableAccessFlags::PRESENT
            | GlobalDescriptorTableAccessFlags::RING_3
            | GlobalDescriptorTableAccessFlags::SYSTEM
            | GlobalDescriptorTableAccessFlags::PRIVILEGE,
        GlobalDescriptorTableEntryFlags::LONG_MODE
    ),
    GlobalDescriptorTableEntry::new(
        GlobalDescriptorTableAccessFlags::PRESENT
            | GlobalDescriptorTableAccessFlags::RING_3
            | GlobalDescriptorTableAccessFlags::TSS_AVAIL,
        GlobalDescriptorTableEntryFlags::NULL
    ),
    GlobalDescriptorTableEntry::NULL
];

#[thread_local]
pub(in self) static INITIALIZE_STACK: [u8; 256] = [0; 256];

// Boot Global Descriptor Table
pub(in self) static mut TEMPORARY_GLOBAL_DESCRIPTOR_TABLE: [GlobalDescriptorTableEntry; TEMPORARY_GLOBAL_DESCRIPTOR_TABLE_ENTRY_COUNT] = [
    GlobalDescriptorTableEntry::NULL,
    GlobalDescriptorTableEntry::new(
        GlobalDescriptorTableAccessFlags::PRESENT
            | GlobalDescriptorTableAccessFlags::RING_0
            | GlobalDescriptorTableAccessFlags::SYSTEM
            | GlobalDescriptorTableAccessFlags::EXECUTABLE
            | GlobalDescriptorTableAccessFlags::PRIVILEGE,
        GlobalDescriptorTableEntryFlags::LONG_MODE
    ),
    GlobalDescriptorTableEntry::new(
        GlobalDescriptorTableAccessFlags::PRESENT
            | GlobalDescriptorTableAccessFlags::RING_0
            | GlobalDescriptorTableAccessFlags::SYSTEM
            | GlobalDescriptorTableAccessFlags::PRIVILEGE,
        GlobalDescriptorTableEntryFlags::LONG_MODE
    ),
    GlobalDescriptorTableEntry::new(
        GlobalDescriptorTableAccessFlags::PRESENT
            | GlobalDescriptorTableAccessFlags::RING_0
            | GlobalDescriptorTableAccessFlags::SYSTEM
            | GlobalDescriptorTableAccessFlags::PRIVILEGE,
        GlobalDescriptorTableEntryFlags::LONG_MODE
    )
];

#[thread_local]
pub static mut PROCESSOR_CONTROL_REGION: ProcessorControlRegion = ProcessorControlRegion::new();

#[repr(C, packed)]
pub(in self) struct GloablDescriptorTableDescriptor {
    size: u16,
    offset: u64
}

impl GloablDescriptorTableDescriptor {
    #[inline]
    pub const fn new(size: u16, offset: u64) -> Self {
        Self {
            size,
            offset
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub(in self) struct GlobalDescriptorTableEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access_byte: u8,
    limit_high_flags: u8,
    base_high: u8
}

impl GlobalDescriptorTableEntry {
    pub(in self) const NULL: Self = Self::new(GlobalDescriptorTableAccessFlags::NULL, GlobalDescriptorTableEntryFlags::NULL);

    pub(in self) const fn new(access_flags: u8, entry_flags: GlobalDescriptorTableEntryFlags) -> Self {
        Self {
            limit_low: 0x00,
            base_low: 0x00,
            base_middle: 0x00,
            access_byte: access_flags,
            limit_high_flags: entry_flags.bits() & 0xF0,
            base_high: 0x00
        }
    }

    pub(in self) fn set_limit(&mut self, limit: u32) {
        self.limit_low = limit as u16;
        self.limit_high_flags = self.limit_high_flags & 0xF0 | ((limit >> 16) as u8) & 0x0F;
    }

    pub(in self) fn set_offset(&mut self, offset: u32) {
        self.base_low = offset as u16;
        self.base_middle = (offset >> 16) as u8;
        self.base_high = (offset >> 24) as u8;
    }

    pub(in self) fn set_raw<T>(&mut self, value: T) {
        unsafe {
            (self as *mut _ as *mut T).write(value)
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct TaskStateSegment {
    reserved: u32,

    rsp: [u64; 3],
    reserved2: u64,

    ist: [u64; 7],
    reserved3: u64,
    reserved4: u64,

    iomap_base: u16
}

impl TaskStateSegment {
    #[inline]
    pub(in self) const fn new() -> Self {
        Self {
            reserved: 0,
            rsp: [0; 3],
            reserved2: 0,
            ist: [0; 7],
            reserved3: 0,
            reserved4: 0,
            iomap_base: 0xFFFF
        }
    }
}

#[repr(C, packed)]
pub struct ProcessorControlRegion {
    pub fs_offset: usize,
    pub user_rsp: usize,
    pub tss: TaskStateSegment
}

impl ProcessorControlRegion {
    pub(in self) const fn new() -> Self {
        Self {
            fs_offset: 0x00,
            user_rsp: 0x00,
            tss: TaskStateSegment::new()
        }
    }
}

pub fn initialize_gloabl_descriptor_table() {
    unsafe {
        let pcr = &mut PROCESSOR_CONTROL_REGION;
        let tss_ptr = &pcr.tss as *const _;

        GLOBAL_DESCRIPTOR_TABLE[GlobalDescriptorTableAccessType::TSS as usize].set_offset(tss_ptr as u32);
        GLOBAL_DESCRIPTOR_TABLE[GlobalDescriptorTableAccessType::TSS as usize].set_limit(mem::size_of::<TaskStateSegment>() as u32);
        GLOBAL_DESCRIPTOR_TABLE[GlobalDescriptorTableAccessType::TSS_HI as usize].set_raw((tss_ptr as u64) >> 32);

        let initialize_stack_address = INITIALIZE_STACK.as_ptr() as usize + INITIALIZE_STACK.len();
        let fault_stack_address = INITIALIZE_STACK.as_ptr() as usize + FAULT_STACK.len();

        PROCESSOR_CONTROL_REGION.tss.rsp[0] = initialize_stack_address as u64;
        PROCESSOR_CONTROL_REGION.tss.ist[0] = fault_stack_address as u64;

        let descriptor = GloablDescriptorTableDescriptor::new(
            (mem::size_of::<[GlobalDescriptorTableEntry; GLOBAL_DESCRIPTOR_TABLE_ENTRY_COUNT]>() - 1) as u16,
            (&GLOBAL_DESCRIPTOR_TABLE as *const _) as u64
        );

        load_global_descriptor_table(&descriptor as *const _);

        io::wrmsr(io::INTEL_ARCHITECTURE_32BIT_GS_BASE, pcr as *mut _ as u64);
        io::wrmsr(io::INTEL_ARCHITECTURE_32BIT_KERNEL_GS_BASE, 0x00);

        GLOBAL_DESCRIPTOR_TABLE[GlobalDescriptorTableAccessType::USER_TLS as usize].set_offset((USER_TCB_OFFSET * 0x1000) as u32);

        load_code_segment_register(SegmentSelector::new(
            GlobalDescriptorTableAccessType::KERNEL_CODE,
            SegmentSelector::RPL_0
        ));

        load_data_segment_register(SegmentSelector::new(
            GlobalDescriptorTableAccessType::KERNEL_DATA, 
            SegmentSelector::RPL_0
        ));

        load_destination_segment_register(SegmentSelector::new(
            GlobalDescriptorTableAccessType::KERNEL_DATA,
            SegmentSelector::RPL_0
        ));

        load_stack_segment_register(SegmentSelector::new(
            GlobalDescriptorTableAccessType::KERNEL_DATA, 
            SegmentSelector::RPL_0
        ));

        load_fs_register(SegmentSelector::new(
            GlobalDescriptorTableAccessType::USER_TLS,
            SegmentSelector::RPL_3
        ));

        load_task_state_segment(SegmentSelector::new(
            GlobalDescriptorTableAccessType::TSS, 
            SegmentSelector::RPL_0
        ));
    }
}

pub fn initialize_temporary_global_descriptor_table() {
    unsafe {
        let descriptor = GloablDescriptorTableDescriptor::new(
            (mem::size_of::<[GlobalDescriptorTableEntry; TEMPORARY_GLOBAL_DESCRIPTOR_TABLE_ENTRY_COUNT]>() - 1) as u16,
            (&TEMPORARY_GLOBAL_DESCRIPTOR_TABLE as *const _) as u64
        );

        load_global_descriptor_table(&descriptor as *const _);
    }

    unsafe {
        load_code_segment_register(SegmentSelector::new(
            GlobalDescriptorTableAccessType::KERNEL_CODE,
            SegmentSelector::RPL_0,
        ));

        load_data_segment_register(SegmentSelector::new(
            GlobalDescriptorTableAccessType::KERNEL_DATA,
            SegmentSelector::RPL_0,
        ));

        load_destination_segment_register(SegmentSelector::new(
            GlobalDescriptorTableAccessType::KERNEL_DATA,
            SegmentSelector::RPL_0,
        ));

        load_fs_register(SegmentSelector::new(
            GlobalDescriptorTableAccessType::KERNEL_DATA,
            SegmentSelector::RPL_0,
        ));

        load_gs_register(SegmentSelector::new(
            GlobalDescriptorTableAccessType::KERNEL_TLS,
            SegmentSelector::RPL_0,
        ));

        load_stack_segment_register(SegmentSelector::new(
            GlobalDescriptorTableAccessType::KERNEL_DATA,
            SegmentSelector::RPL_0,
        ));
    }
}

#[inline(always)]
pub(in self) unsafe fn load_global_descriptor_table(descriptor: *const GloablDescriptorTableDescriptor) {
    asm!("lgdt [rdi]", in("rdi") descriptor, options(nostack))
}

#[inline(always)]
pub(in self) unsafe fn load_task_state_segment(selector: SegmentSelector) {
    asm!("ltr [rdi]", in("rdi") selector.bits(), options(nomem, nostack))
}

#[inline(always)]
pub(in self) unsafe fn load_code_segment_register(selector: SegmentSelector) {
    asm!(
        "push {selector}",
        "lea {tmp}, [1f + rip]",
        "push {tmp}",
        "retfq",
        "1:",
        selector = in(reg) u64::from(selector.bits()),
        tmp = lateout(reg) _
    )
}

#[inline(always)]
pub(in self) unsafe fn load_data_segment_register(selector: SegmentSelector) {
    asm!("mov ds, {0:x}", in(reg) selector.bits())
}

#[inline(always)]
pub(in self) unsafe fn load_destination_segment_register(selector: SegmentSelector) {
    asm!("mov es, {0:x}", in(reg) selector.bits(), options(nomem, nostack))
}

#[inline(always)]
pub(in self) unsafe fn load_fs_register(selector: SegmentSelector) {
    asm!("mov fs, {0:x}", in(reg) selector.bits(), options(nomem, nostack))
}

#[inline(always)]
pub(in self) unsafe fn load_gs_register(selector: SegmentSelector) {
    asm!("mov gs, {0:x}", in(reg) selector.bits(), options(nomem, nostack))
}

#[inline(always)]
pub(in self) unsafe fn load_stack_segment_register(selector: SegmentSelector) {
    asm!("mov ss, {0:x}", in(reg) selector.bits(), options(nomem, nostack))
}
