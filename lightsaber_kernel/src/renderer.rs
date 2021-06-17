use core::fmt::{
    self,
    Write
};

use spin::{
    Mutex,
    Once
};

use lightsaber_graphics::{
    debug_render::{
        colour::ColourCode,
        render::DebugRender
    },
    Framebuffer
};

pub(in self) static DEBUG_RENDERER: Once<Mutex<DebugRender>> = Once::new();

pub fn initialize_renderer(framebuffer: &'static mut Framebuffer) {
    let information = framebuffer.information();
    let buffer = framebuffer.buffer_mut();

    let mut renderer = DebugRender::new(buffer, information);
    renderer.clear_screen();

    DEBUG_RENDERER.call_once(|| Mutex::new(renderer));
}

pub fn clear_screen() {
    DEBUG_RENDERER.get().unwrap().lock().clear_screen();
}

#[inline]
pub fn is_initialized() -> bool {
    DEBUG_RENDERER.get().is_some()
}

pub fn set_colour_code(colour_code: ColourCode) {
    DEBUG_RENDERER.get().unwrap().lock().set_colour_code(colour_code);
}

pub fn __print(args: fmt::Arguments) {
    DEBUG_RENDERER.get().unwrap().lock().write_fmt(args).unwrap();
}

pub macro print {
    ($($arg:tt)*) => {
        $crate::renderer::__print(format_args!($($arg)*));
    }
}

pub macro println {
    () => {
        $crate::renderer::print!("\n");
    },
    ($($arg:tt)*) => {
        $crate::renderer::print!("{}\n", format_args!($($arg)*));
    }
}

pub macro dbg {
    () => {
        $crate::renderer::println!("[{}:{}]", $core::file!(), $core::line!());
    },
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::renderer::println!("[{}:{}] {} = {:#?}",
                    core::file!(), core::line!(), core::stringify!($val), &tmp);
                tmp
            }
        }
    },
    ($($val:expr),+ $(,)?) => {
        ($($crate::renderer::dbg!($val)),+,)
    }
}
