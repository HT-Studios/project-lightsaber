use core::fmt;

use lightsaber_util::libcore_ext::ops::Reset;

use crate::{
    debug_render::colour::{
        Colour,
        ColourCode
    },
    font8x16::{
        basic,
        unicode::UnicodeFonts
    },
    FramebufferInformation
};

pub struct DebugRender<'buffer> {
    buffer: &'buffer mut [u8],
    information: FramebufferInformation,
    x_position: usize,
    y_position: usize,
    colour: ColourCode
}

impl<'buffer> DebugRender<'buffer> {
    #[inline]
    pub fn new(buffer: &'buffer mut [u8], information: FramebufferInformation) -> Self {
        Self {
            buffer,
            information,
            x_position: usize::default(),
            y_position: usize::default(),
            colour: ColourCode::new(Colour::WHITE, Colour::BLACK)
        }
    }

    pub fn clear_screen(&mut self) {
        self.x_position.reset();
        self.y_position.reset();

        self.buffer.fill(self.colour.background().inner() as u8);
    }

    pub fn height(&self) -> usize {
        self.information.vertical_resolution
    }

    pub fn put_bytes(&mut self, bytes: &[u8]) {
        bytes.iter().enumerate().for_each(|(y, byte)| {
            (0..8).enumerate().for_each(|(x, bit)| {
                if *byte & ( 1 << bit) == 0 {
                    // BACKGROUND
                    self.put_pixel(self.x_position + x, self.y_position + y, self.colour.background());
                }
                else {
                    // FOREGROUND
                    self.put_pixel(self.x_position + x, self.y_position + y, self.colour.foreground());
                }
            });
        });

        self.x_position += 8;
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, colour: Colour) {
        let pixel_offset = y * self.information.stride + x;
        let colour = [
            colour.red(),
            colour.green(),
            colour.blue(),
            colour.alpha()
        ];
        let bytes_per_pixel = self.information.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;

        self.buffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&colour[..bytes_per_pixel]);
    }

    pub fn set_colour_code(&mut self, other: ColourCode) {
        if self.colour == other {
            return;
        }

        self.colour = other;
    }

    pub fn width(&self) -> usize {
        self.information.horizontal_resolution
    }

    pub fn write_character(&mut self, character: char) {
        match character {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            _ => {
                let mut character_ = basic::BASIC_FONTS.get(character)
                    .expect("Character to print is not included in Basic Unicode (Code Points U+0000 to U+007F).");
                character_.iter_mut().for_each(|u8_val| *u8_val = u8_val.reverse_bits());

                if self.x_position >= self.width() {
                    self.newline();
                }

                if self.y_position >= (self.height() - 20) {
                    self.clear_screen();
                }

                self.put_bytes(&character_);
            }
        }
    }

    pub fn write_refstr(&mut self, refstr: &str) {
        refstr.chars().for_each(|character| {
            self.write_character(character)
        });
    }

    pub(in self) fn carriage_return(&mut self) {
        self.x_position.reset();
    }

    pub(in self) fn newline(&mut self) {
        self.y_position += 20;
        self.carriage_return();
    }
}

impl<'buffer> fmt::Write for DebugRender<'buffer> {
    fn write_str(&mut self, refstr: &str) -> fmt::Result {
        self.write_refstr(refstr);

        Ok(())
    }
}

unsafe impl<'buffer> Send for DebugRender<'buffer> { }
unsafe impl<'buffer> Sync for DebugRender<'buffer> { }
