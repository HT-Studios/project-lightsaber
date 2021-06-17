use core::mem;

use lightsaber_util::libcore_ext::io;

pub mod entry_flags;
pub mod registers;
pub mod stack_frame;

use crate::architecture::amd64::interrupts::exceptions;
pub(in self) use entry_flags::InterruptDescriptorTableFlags;

pub(in crate) const INTERRUPT_DESCRIPTOR_TABLE_ENTRIES: usize = 256;

pub(in crate) const PIC1_COMMAND: u16 = 0x20;
pub(in crate) const PIC1_DATA: u16 = 0x21;

pub(in crate) const PIC2_COMMAND: u16 = 0xA0;
pub(in crate) const PIC2_DATA: u16 = 0xA1;

pub(in crate) const PIC_EOI: u8 = 0x20;

pub(in crate) const ICW1_INIT: u8 = 0x10;
pub(in crate) const ICW1_ICW4: u8 = 0x01;
pub(in crate) const ICW4_8086: u8 = 0x01;

pub(in crate) type InterruptHandlerFn = unsafe extern "C" fn();

pub(in self) static mut INTERRUPT_DESCRIPTOR_TABLE: [InterruptDescriptorTableEntry; INTERRUPT_DESCRIPTOR_TABLE_ENTRIES] =
    [InterruptDescriptorTableEntry::NULL; INTERRUPT_DESCRIPTOR_TABLE_ENTRIES];

#[repr(C, packed)]
pub(in self) struct InterruptDescriptorTableDescriptor {
    size: u16,
    offset: u64
}

impl InterruptDescriptorTableDescriptor {
    #[inline]
    pub(in self) const fn new(size: u16, offset: u64) -> Self {
        Self {
            size,
            offset
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub(in self) struct InterruptDescriptorTableEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_middle: u16,
    offset_high: u32,
    ignore: u32
}

impl InterruptDescriptorTableEntry {
    pub(in self) const NULL: Self = Self {
        offset_low: 0x00,
        selector: 0x00,
        ist: 0x00,
        type_attr: 0x00,
        offset_middle: 0x00,
        offset_high: 0x00,
        ignore: 0x00,
    };

    pub(in crate) fn set_fn(&mut self, handler: InterruptHandlerFn) {
        self.set_flags(InterruptDescriptorTableFlags::PRESENT | InterruptDescriptorTableFlags::RING_0 | InterruptDescriptorTableFlags::INTERRUPT);
        self.set_offset(8, handler as usize);
    }

    pub(in self) fn set_flags(&mut self, flags: InterruptDescriptorTableFlags) {
        self.type_attr = flags.bits;
    }

    pub(in self) fn set_offset(&mut self, selector: u16, base: usize) {
        self.selector = selector;
        self.offset_low = base as u16;
        self.offset_middle = (base >> 16) as u16;
        self.offset_high = (base >> 32) as u32;
    }
}

pub unsafe fn disable_pic() {
    io::outb(PIC1_DATA, 0xFF);
    io::wait();

    io::outb(PIC2_DATA, 0xFF);
    io::wait();
}

#[inline]
pub unsafe fn end_pic1() {
    io::outb(PIC1_COMMAND, PIC_EOI);
}

#[inline]
pub unsafe fn end_pic2() {
    io::outb(PIC2_COMMAND, PIC_EOI);
    io::outb(PIC1_COMMAND, PIC_EOI);
}

pub unsafe fn load_pic() {
    let (a1, a2): (u8, u8);

    a1 = io::inb(PIC1_DATA);
    io::wait();

    a2 = io::inb(PIC2_DATA);
    io::wait();

    io::outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
    io::wait();
    io::outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
    io::wait();

    io::outb(PIC1_DATA, 0x20);
    io::wait();
    io::outb(PIC2_DATA, 0x28);
    io::wait();

    io::outb(PIC1_DATA, 4);
    io::wait();
    io::outb(PIC2_DATA, 2);
    io::wait();

    io::outb(PIC1_DATA, ICW4_8086);
    io::wait();
    io::outb(PIC2_DATA, ICW4_8086);
    io::wait();

    io::outb(PIC1_DATA, a1);
    io::wait();
    io::outb(PIC2_DATA, a2);
    io::wait();
}

pub fn initialize_interrupt_descriptor_table() {
    unsafe {
        INTERRUPT_DESCRIPTOR_TABLE[0].set_fn(exceptions::division_by_zero);
        INTERRUPT_DESCRIPTOR_TABLE[1].set_fn(exceptions::debug);
        INTERRUPT_DESCRIPTOR_TABLE[2].set_fn(exceptions::nonmaskable_interrupt);
        INTERRUPT_DESCRIPTOR_TABLE[3].set_fn(exceptions::breakpoint);
        INTERRUPT_DESCRIPTOR_TABLE[4].set_fn(exceptions::overflow);
        INTERRUPT_DESCRIPTOR_TABLE[5].set_fn(exceptions::bound_range_exceeded);
        INTERRUPT_DESCRIPTOR_TABLE[6].set_fn(exceptions::invalid_opcode);
        INTERRUPT_DESCRIPTOR_TABLE[7].set_fn(exceptions::device_unavailable);
        INTERRUPT_DESCRIPTOR_TABLE[8].set_fn(exceptions::double_fault);

        INTERRUPT_DESCRIPTOR_TABLE[10].set_fn(exceptions::invalid_task_state_segment);
        INTERRUPT_DESCRIPTOR_TABLE[11].set_fn(exceptions::segment_not_present);
        INTERRUPT_DESCRIPTOR_TABLE[12].set_fn(exceptions::stack_segment_fault);
        INTERRUPT_DESCRIPTOR_TABLE[13].set_fn(exceptions::general_protection_fault);

        INTERRUPT_DESCRIPTOR_TABLE[14].set_flags(InterruptDescriptorTableFlags::PRESENT | InterruptDescriptorTableFlags::RING_0 | InterruptDescriptorTableFlags::INTERRUPT);
        INTERRUPT_DESCRIPTOR_TABLE[14].set_offset(8, exceptions::page_fault as usize);

        INTERRUPT_DESCRIPTOR_TABLE[16].set_fn(exceptions::floating_point_exception);
        INTERRUPT_DESCRIPTOR_TABLE[17].set_fn(exceptions::alignment_check);
        INTERRUPT_DESCRIPTOR_TABLE[18].set_fn(exceptions::machine_check);
        INTERRUPT_DESCRIPTOR_TABLE[19].set_fn(exceptions::simd_floating_point_exception);
        INTERRUPT_DESCRIPTOR_TABLE[20].set_fn(exceptions::virtualization_fault);
        
        INTERRUPT_DESCRIPTOR_TABLE[30].set_fn(exceptions::security_exception);

        let descriptor = InterruptDescriptorTableDescriptor::new(
            ((INTERRUPT_DESCRIPTOR_TABLE.len() * mem::size_of::<InterruptDescriptorTableEntry>()) - 1) as u16,
            (&INTERRUPT_DESCRIPTOR_TABLE as *const _) as u64
        );

        load_interrupt_descriptor_table(&descriptor as *const _);
        load_pic();
    }
}

#[inline]
pub(in self) unsafe fn load_interrupt_descriptor_table(descriptor: *const InterruptDescriptorTableDescriptor) {
    asm!("lidt [{}]", in(reg) descriptor, options(nostack))
}
