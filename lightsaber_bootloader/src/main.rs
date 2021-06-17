#![no_std]
#![no_main]

#![feature(abi_efiapi)]
#![feature(asm)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_slice)]
#![feature(never_type)]
#![feature(try_trait_v2)]

extern crate rlibc;

use core::{
    mem,
    slice
};

use uefi::{
    prelude::{
        Boot,
        entry,
        Handle,
        ResultExt,
        Status,
        SystemTable
    },
    proto::console::gop::{
        GraphicsOutput,
        PixelFormat
    },
    table::boot::{
        MemoryDescriptor,
        MemoryType
    },
    table::cfg
};

use x86_64::PhysAddr;

use lightsaber_graphics::{
    FramebufferInformation,
};

mod load;
mod logger;
mod paging;
mod unwind;

use crate::{
    load::SystemInformation,
    paging::BootFrameAllocator
};

pub const KERNEL_ELF_PATH: &'static str = r"\efi\kernel\kernel.elf";

fn initialize_display(system_table: &SystemTable<Boot>) -> (PhysAddr, FramebufferInformation) {
    let gop = system_table
        .boot_services()
        .locate_protocol::<GraphicsOutput>()
        .expect_success("Failed to locate GOP");

    let gop = unsafe { &mut *gop.get() };

    let mode_info = gop.current_mode_info();
    let mut framebuffer = gop.frame_buffer();

    let slice = unsafe { slice::from_raw_parts_mut(framebuffer.as_mut_ptr(), framebuffer.size()) };

    let info = FramebufferInformation {
        len_bytes: framebuffer.size(),
        horizontal_resolution: mode_info.resolution().0,
        vertical_resolution: mode_info.resolution().1,
        pixel_format: match mode_info.pixel_format() {
            PixelFormat::Rgb => lightsaber_graphics::PixelFormat::RedGreenBlue,
            PixelFormat::Bgr => lightsaber_graphics::PixelFormat::BlueGreenRed,
            PixelFormat::Bitmask | PixelFormat::BltOnly => {
                panic!("Bitmask and BltOnly framebuffers are not supported.")
            }
        },
        bytes_per_pixel: 4,
        stride: mode_info.stride(),
    };

    let global_logger = logger::MutexedLogger::new(lightsaber_graphics::debug_render::render::DebugRender::new(slice, info));
    let locked_logger = logger::LOGGER.call_once(|| global_logger);

    log::set_logger(locked_logger).expect("Failed to set the global logger.");
    log::set_max_level(log::LevelFilter::Info);

    (PhysAddr::new(framebuffer.as_mut_ptr() as u64), info)
}

#[entry]
fn efi_main(image: Handle, system_table: SystemTable<Boot>) -> Status {
    let (framebuffer_address, framebuffer_info) = initialize_display(&system_table);
    log::info!("Using framebuffer at: {:#x}.", framebuffer_address);

    let kernel_bytes = load::load_file(system_table.boot_services(), KERNEL_ELF_PATH);

    let mmap_storage = {
        let max_mmap_size =
            system_table.boot_services().memory_map_size() + 8 * mem::size_of::<MemoryDescriptor>();

        let ptr = system_table
            .boot_services()
            .allocate_pool(MemoryType::LOADER_DATA, max_mmap_size)
            .expect_success("Failed to allocate pool.");

        unsafe { slice::from_raw_parts_mut(ptr, max_mmap_size) }
    };

    log::info!("Exiting boot services.");

    let (system_table, memory_map) = system_table
        .exit_boot_services(image, mmap_storage)
        .expect_success("Failed to exit boot services.");

    let mut frame_allocator = BootFrameAllocator::new(memory_map.copied());
    let page_tables = paging::initialize_paging(&mut frame_allocator);

    let mut config_entries = system_table.config_table().iter();

    let rsdp_address = config_entries
        .find(|entry| matches!(entry.guid, cfg::ACPI_GUID | cfg::ACPI2_GUID))
        .map(|entry| PhysAddr::new(entry.address as u64))
        .expect("Lightsaber requires an ACPI-compatible system.");

    let system_info = SystemInformation {
        framebuffer_address,
        framebuffer_information: framebuffer_info,
        rsdp_address
    };

    load::load_and_switch_to_kernel(frame_allocator, page_tables, kernel_bytes, system_info);
}
