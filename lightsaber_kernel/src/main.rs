#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(allocator_api)]
#![feature(asm)]
#![feature(decl_macro)]
#![feature(global_asm)]
#![feature(lang_items)]
#![feature(panic_info_message)]
#![feature(thread_local)]

extern crate alloc;
extern crate rlibc;

mod architecture;
mod drivers;
mod logger;
mod renderer;
mod time;
mod tls;
mod unwind;

use spin::Once;

use x86_64::VirtAddr;

use lightsaber_bootloader::{
    BootInformation,
    UnwindInformation
};

use lightsaber_util::libcore_ext::io;

pub(in self) const WELCOME_MESSAGE:&str = r"Welcome to Lightsaber - a Unix-based operating system written in Rust.

This project is maintained by HTGAzureX1212., and is currently in its
earliest stages.

================================
Operating System Information
================================
Kernel Version: 1.0
OS Build: 44.0
Git Commit Hash: 3dfb9cf366060ac650e465e41ca2352a313bad96
";

#[global_allocator]
pub(in self) static ALLOCATOR: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();

pub(in self) static mut PHYSICAL_MEMORY_OFFSET: VirtAddr = VirtAddr::zero();
pub(in self) static UNWIND_INFORMATION: Once<UnwindInformation> = Once::new();

#[export_name = "_start"]
extern "C" fn kernel_main(boot_information: &'static mut BootInformation) -> ! {
    unsafe {
        #[cfg(target_arch = "x86_64")]
            architecture::amd64::interrupts::disable_interrupts();
    }

    drivers::uart16550::initialize();

    unsafe {
        PHYSICAL_MEMORY_OFFSET = boot_information.physical_memory_offset
    };

    UNWIND_INFORMATION.call_once(|| boot_information.unwind_information);

    renderer::initialize_renderer(&mut boot_information.framebuffer);
    logger::initialize_logger();

    #[cfg(target_arch = "x86_64")]
        architecture::amd64::gdt::initialize_temporary_global_descriptor_table();
    log::info!("Initialized temporary Boot Global Descriptor Table.");

    #[cfg(target_arch = "x86_64")]
        architecture::amd64::interrupts::idt::initialize_interrupt_descriptor_table();
    log::info!("Initialized the Interrupt Descriptor Table");

    time::initialize_programmable_interval_timer();
    log::info!("Initialized the Programmable Interval Timer.");

    #[cfg(target_arch = "x86_64")]
        let (mut offset_table, mut frame_allocator) =
        architecture::amd64::memory::paging::initialize_paging(boot_information.physical_memory_offset, &boot_information.memory_regions);
    log::info!("Initialized paging.");

    #[cfg(target_arch = "x86_64")]
        architecture::amd64::memory::heap::initialize_heap(&mut offset_table, &mut frame_allocator)
        .expect("Failed to initialize the heap.");
    log::info!("Initialized the heap.");

    tls::initialize_tls();
    log::info!("Initialized the Thread-Local Storage.");

    #[cfg(target_arch = "x86_64")]
        architecture::amd64::gdt::initialize_gloabl_descriptor_table();
    log::info!("Initialized Global Descriptor Table.");

    #[cfg(target_arch = "x86_64")]
        unsafe {
        //io::outb(
        //    architecture::amd64::interrupts::idt::PIC1_DATA,
        //    0b11111000
        //);
        //io::outb(
        //    architecture::amd64::interrupts::idt::PIC2_DATA,
        //    0b11101111
        //);

        //architecture::amd64::interrupts::enable_interrupts();
    }

    log::info!("The kernel is initialized.");

    renderer::clear_screen();

    renderer::println!("{}", WELCOME_MESSAGE);
    renderer::println!("defaultuser@lightsaber:~$ rickroll");
    renderer::println!("Never gonna give you up, never gonna let you down.");
    renderer::println!();
    renderer::println!("defaultuser@lightsaber:~$");

    unsafe {
        loop {
            #[cfg(target_arch = "x86_64")]
                architecture::amd64::interrupts::halt();
        }
    }
}

#[cfg(not(target_arch = "x86_64"))]
compile_error!("The operating system currently only supports the x86_64/AMD64 architecture.");
