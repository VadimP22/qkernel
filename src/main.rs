#![no_std]
#![no_main]
#![feature(const_mut_refs)]

mod common;
mod console;

use console::writer::{VGAWriter, VGA_WRITER};
use core::fmt::Write;
use core::panic::PanicInfo;

use crate::console::logger::{Logger, LOGGER};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        _ = write!(VGA_WRITER.lock(), "{}", info);
    }
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut vga_writer = VGAWriter::default();
    let mut logger = Logger::new();
    unsafe {
        VGA_WRITER.lock().initialize(&mut vga_writer);
        LOGGER.lock().initialize(&mut logger);
        log_trace!("Initialized: VGA_WRITER, LOGGER");
    }

    loop {}
}
