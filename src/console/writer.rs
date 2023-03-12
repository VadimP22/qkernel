use core::fmt::{self, Write};

use spin::Mutex;

use crate::common::initializible::Initializible;

use super::vga_text_buffer::{
    VGABuffer, VGAChar, VGAColor, VGAColorCode, VGA_BUFFER_HEIGHT, VGA_BUFFER_WIDTH,
};

#[derive(Debug)]
pub struct VGAWriter {
    pub current_column: usize,
    pub base_color: VGAColor,
    pub current_color: VGAColor,
    pub vga_buffer: &'static mut VGABuffer,
}

impl VGAWriter {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),

            byte => {
                if self.current_column >= VGA_BUFFER_WIDTH {
                    self.new_line();
                }

                self.vga_buffer.chars[VGA_BUFFER_HEIGHT - 1][self.current_column] = VGAChar {
                    ascii_character: byte,
                    color: self.current_color,
                };

                self.current_column += 1;
            }
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..VGA_BUFFER_HEIGHT {
            for col in 0..VGA_BUFFER_WIDTH {
                let char = self.vga_buffer.chars[row][col];
                self.vga_buffer.chars[row - 1][col] = char;
            }
        }
        self.clean_row(VGA_BUFFER_HEIGHT - 1);
        self.current_column = 0;
    }

    fn clean_row(&mut self, row: usize) {
        for col in &mut self.vga_buffer.chars[row] {
            col.ascii_character = ' ' as u8;
        }
    }

    pub fn set_color(&mut self, new_color: VGAColor) {
        self.current_color = new_color;
    }

    pub fn reset_color(&mut self) {
        self.current_color = self.base_color;
    }
}

impl Default for VGAWriter {
    fn default() -> Self {
        return VGAWriter {
            current_column: 0,
            base_color: VGAColor::new(VGAColorCode::LightGray, VGAColorCode::Black),
            current_color: VGAColor::new(VGAColorCode::LightGray, VGAColorCode::Black),
            vga_buffer: unsafe { &mut *(0xb8000 as *mut VGABuffer) },
        };
    }
}

impl fmt::Write for VGAWriter {
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.write_byte(c as u8);
        return Ok(());
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        return fmt::write(self, args);
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        return Ok(());
    }
}

pub static mut VGA_WRITER: Mutex<Initializible<VGAWriter>> = Mutex::new(Initializible::new());

#[doc(hidden)]
pub fn _vga_print(args: fmt::Arguments) {
    unsafe {
        VGA_WRITER.lock().write_fmt(args).unwrap();
    }
}
