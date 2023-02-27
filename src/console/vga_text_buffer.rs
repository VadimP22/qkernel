pub const VGA_BUFFER_HEIGHT: usize = 25;
pub const VGA_BUFFER_WIDTH: usize = 80;

/// 4-bit vga color.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VGAColorCode {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VGAColor(u8);

impl VGAColor {
    pub const fn new(foreground: VGAColorCode, background: VGAColorCode) -> VGAColor {
        return VGAColor((background as u8) << 4 | (foreground as u8));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VGAChar {
    pub ascii_character: u8,
    pub color: VGAColor,
}

#[repr(transparent)]
#[derive(Debug)]
pub struct VGABuffer {
    pub chars: [[VGAChar; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}
