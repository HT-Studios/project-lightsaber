use core::panic::PanicInfo;

#[panic_handler]
pub unsafe extern "C" fn rust_begin_unwind(panic_info: &PanicInfo) -> ! {
    log::error!("{}", panic_info);

    asm!("cli");

    loop {
        asm!("hlt");
    }
}
