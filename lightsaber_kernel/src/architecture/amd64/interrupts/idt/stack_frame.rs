use x86_64::VirtAddr;

use crate::architecture::amd64::interrupts::idt::registers::{
    IretRegisters,
    PreservedRegisters,
    ScratchRegisters
};

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct InterruptErrorStack {
    pub code: usize,
    pub stack: InterruptStack,
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct InterruptStack {
    pub fs: usize,
    pub preserved: PreservedRegisters,
    pub scratch: ScratchRegisters,
    pub iret: IretRegisters,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct InterruptStackFrame {
    pub insturction_ptr: VirtAddr,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_ptr: VirtAddr,
    pub stack_segment: u64
}
