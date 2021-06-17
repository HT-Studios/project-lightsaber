#![no_std]

use core::slice;

pub mod debug_render;
pub mod font8x16;

#[derive(Debug)]
#[repr(C)]
pub struct Framebuffer {
    pub buffer_start: u64,
    pub buffer_len_bytes: usize,
    pub information: FramebufferInformation
}

impl Framebuffer {
    pub fn buffer(&self) -> &[u8] {
        unsafe { self.create_buffer_mut() }
    }

    pub fn buffer_mut(&self) -> &mut [u8] {
        unsafe { self.create_buffer_mut() }
    }

    pub fn information(&self) -> FramebufferInformation {
        self.information
    }

    pub(in self) unsafe fn create_buffer_mut<'buffer>(&self) -> &'buffer mut [u8] {
        slice::from_raw_parts_mut(self.buffer_start as *mut u8, self.buffer_len_bytes)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct FramebufferInformation {
    pub len_bytes: usize,
    pub horizontal_resolution: usize,
    pub vertical_resolution: usize,
    pub pixel_format: PixelFormat,
    pub bytes_per_pixel: usize,
    pub stride: usize
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum PixelFormat {
    RedGreenBlue,
    BlueGreenRed,
    Unsigned8BitInteger
}
