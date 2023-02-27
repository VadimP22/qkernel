#![no_std]
#![no_main]
#![feature(const_mut_refs)]

mod common;
mod console;

use console::writer::{VGAWriter, VGA_WRITER};
use core::fmt::Write;
use core::panic::PanicInfo;

use crate::console::vga_text_buffer::VGAColorCode;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut vga_writer = VGAWriter::default();
    unsafe {
        VGA_WRITER.lock().initialize(&mut vga_writer);
        write!(VGA_WRITER.lock(), "Something {:?} and {}", VGAColorCode::Black, "cool").unwrap();
    }

    loop {}
}
