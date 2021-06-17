use core::panic::PanicInfo;

use x86_64::VirtAddr;

use crate::{
    architecture,
    drivers::uart16550,
    renderer,
};

#[panic_handler]
pub extern "C" fn rust_begin_unwind(panic_info: &PanicInfo<'_>) -> ! {
    let default_panic_message = &format_args!("");
    let panic_message = panic_info.message().unwrap_or(default_panic_message);

    if renderer::is_initialized() {
        renderer::clear_screen();

        log::error!("Unexpected Kernel Panic");
        log::error!("{}", panic_info.location().unwrap());
        log::error!("{}", panic_message);
    }
    else {
        uart16550::serial_println!("Unexpected Kernel Panic");
        uart16550::serial_println!("Kernel panicked before the debug renderer is initialized");
        uart16550::serial_println!("{}", panic_info.location().unwrap());
        uart16550::serial_println!("{}", panic_message);
    }

    unsafe {
        #[cfg(target_arch = "x86_64")]
        architecture::amd64::interrupts::disable_interrupts();

        loop {
            #[cfg(target_arch = "x86_64")]
            architecture::amd64::interrupts::halt();
        }
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() { }

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}