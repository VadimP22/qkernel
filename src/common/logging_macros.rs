#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::writer::_vga_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => (
        unsafe {
            crate::console::logger::LOGGER.lock().log("[T] ",
                crate::console::vga_text_buffer::VGAColorCode::LightBlue,
                format_args!($($arg)*)
            );
        }
    );
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => (
        unsafe {
            crate::console::logger::LOGGER.lock().log("[D] ",
                crate::console::vga_text_buffer::VGAColorCode::LightGreen,
                format_args!($($arg)*)
            );
        }
    );
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => (
        unsafe {
            crate::console::logger::LOGGER.lock().log("[I] ",
                crate::console::vga_text_buffer::VGAColorCode::LightGray,
                format_args!($($arg)*)
            );
        }
    );
}

#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => (
        unsafe {
            crate::console::logger::LOGGER.lock().log("[W] ",
                crate::console::vga_text_buffer::VGAColorCode::Yellow,
                format_args!($($arg)*)
            );
        }
    );
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => (
        unsafe {
            crate::console::logger::LOGGER.lock().log("[E] ",
                crate::console::vga_text_buffer::VGAColorCode::LightRed,
                format_args!($($arg)*)
            );
        }
    );
}

#[macro_export]
macro_rules! log_critical {
    ($($arg:tt)*) => (
        unsafe {
            crate::console::logger::LOGGER.lock().log("[C] ",
                crate::console::vga_text_buffer::VGAColorCode::Pink,
                format_args!($($arg)*)
            );
        }
    );
}
