pub mod exceptions;
pub mod idt;

#[inline(always)]
pub unsafe fn disable_interrupts() {
    asm!("cli", options(nomem, nostack))
}

#[inline(always)]
pub unsafe fn enable_interrupts() {
    asm!("sti", options(nomem, nostack))
}

#[inline(always)]
pub unsafe fn enable_interrupts_halt() {
    enable_interrupts();
    halt();
}

#[inline(always)]
pub unsafe fn halt() {
    asm!("hlt", options(nomem, nostack))
}

#[inline(always)]
pub fn pause() {
    unsafe {
        asm!("pause", options(nomem, nostack));
    }
}
