use core::{
    fmt::{
        self,
        Write
    }
};

use spin::{
    Mutex,
    Once
};

use lightsaber_util::libcore_ext::io;

pub mod line_status;

pub(in self) use line_status::LineStatus;

pub(in self) static COM_1: Once<Mutex<SerialPort>> = Once::new();

#[repr(transparent)]
pub struct SerialPort(u16);

impl SerialPort {
    #[inline(always)]
    pub const fn new(port: u16) -> Self {
        Self(port)
    }

    pub unsafe fn initialize(self) -> Self {
        io::outb(self.0 + 1, 0x00);

        io::outb(self.0 + 3, 0x80);

        io::outb(self.0, 0x03);
        io::outb(self.0 + 1, 0x00);

        io::outb(self.0 + 3, 0x03);

        io::outb(self.0 + 2, 0xC7);

        io::outb(self.0 + 4, 0x0B);

        io::outb(self.0 + 1, 0x01);

        self
    }

    pub fn line_status(&self) -> LineStatus {
        unsafe {
            let status = io::inb(self.0 + 5);

            LineStatus::from_bits_truncate(status)
        }
    }

    pub fn send_byte(&mut self, byte: u8) {
        unsafe {
            match byte {
                8 | 0x7F => {
                    self.wait_for_line_status(LineStatus::OUTPUT_EMPTY);
                    io::outb(self.0, 8);

                    self.wait_for_line_status(LineStatus::OUTPUT_EMPTY);
                    io::outb(self.0, b' ');

                    self.wait_for_line_status(LineStatus::OUTPUT_EMPTY);
                    io::outb(self.0, 8);
                }
                _ => {
                    self.wait_for_line_status(LineStatus::OUTPUT_EMPTY);
                    io::outb(self.0, byte)
                }
            }
        }
    }

    pub(in self) fn wait_for_line_status(&self, line_status: LineStatus) {
        while !self.line_status().contains(line_status) {
            core::hint::spin_loop()
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for byte in string.bytes() {
            self.send_byte(byte);
        }

        Ok(())
    }
}

pub fn initialize() {
    unsafe {
        let com_1 = SerialPort::new(0x3F8).initialize();

        COM_1.call_once(move || Mutex::new(com_1));
    }
}

pub fn __serial_print(args: fmt::Arguments) {
    COM_1
        .get()
        .unwrap()
        .lock()
        .write_fmt(args)
        .expect("Failed to write to the COM1 port");
}

pub macro serial_print($($arg:tt)*) {
    crate::drivers::uart16550::__serial_print(format_args!($($arg)*));
}

pub macro serial_println {
    () => ($crate::drivers::uart16550::serial_print!("\n")),
    ($($arg:tt)*) => ($crate::drivers::uart16550::serial_print!("{}\n", format_args!($($arg)*)))
}
