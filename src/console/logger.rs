use core::fmt::{self, Write};

use spin::Mutex;

use crate::common::initializible::Initializible;

use super::{
    vga_text_buffer::{VGAColor, VGAColorCode},
    writer::VGA_WRITER,
};

pub struct Logger {
    colored: bool,
    log_serial: bool,
    log_vga: bool,
}

impl Logger {
    pub const fn new() -> Logger {
        return Logger {
            colored: true,
            log_serial: false,
            log_vga: true,
        };
    }

    pub fn log(&mut self, prefix: &str, foreground_color: VGAColorCode, message: fmt::Arguments) {
        unsafe {
            let mut writer = VGA_WRITER.lock();

            if self.colored {
                writer.set_color(VGAColor::new(foreground_color, VGAColorCode::Black));
            }
            writer.write_string(prefix);
            writer.write_fmt(message).unwrap();
            writer.write_byte('\n' as u8);
            writer.reset_color()
        }
    }
}

pub static mut LOGGER: Mutex<Initializible<Logger>> = Mutex::new(Initializible::new());
