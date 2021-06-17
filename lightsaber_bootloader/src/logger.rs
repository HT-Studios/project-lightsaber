use core::fmt::Write;

use log::{
    Level,
    Metadata,
    Record
};

use spin::{
    Mutex,
    Once
};

use lightsaber_graphics::debug_render::{
    colour::{
        Colour,
        ColourCode
    },
    render::DebugRender
};

pub static LOGGER: Once<MutexedLogger> = Once::new();


pub struct MutexedLogger<'buffer>(Mutex<DebugRender<'buffer>>);

impl<'buffer> MutexedLogger<'buffer> {
    #[inline(always)]
    pub fn new(mut inner: DebugRender<'buffer>) -> Self {
        inner.clear_screen();

        Self(Mutex::new(inner))
    }
}

impl<'buffer> log::Log for MutexedLogger<'buffer> {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let self_locked = &mut self.0.lock();
        self_locked.set_colour_code(ColourCode::new(Colour::WHITE, Colour::BLACK));
        write!(self_locked, "[ ").expect("Failed to write to the framebuffer.");

        match record.level() {
            Level::Error => self_locked.set_colour_code(ColourCode::new(Colour::from_hex(0xFF0000), Colour::BLACK)),
            Level::Warn => self_locked.set_colour_code(ColourCode::new(Colour::from_hex(0xDEDB18), Colour::BLACK)),
            Level::Info => self_locked.set_colour_code(ColourCode::new(Colour::from_hex(0x21AD11), Colour::BLACK)),
            Level::Debug => self_locked.set_colour_code(ColourCode::new(Colour::from_hex(0x116AAD), Colour::BLACK)),
            Level::Trace => self_locked.set_colour_code(ColourCode::new(Colour::from_hex(0x4F524E), Colour::BLACK))
        };

        write!(self_locked, "{}", record.level()).expect("Failed to write to framebuffer.");

        self_locked.set_colour_code(ColourCode::new(Colour::WHITE, Colour::BLACK));
        write!(self_locked, " ]    - {}\n", record.args()).expect("Failed to write to the framebuffer.");
    }

    fn flush(&self) { }
}
