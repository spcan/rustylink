//! STLink modes

pub const DFU            : u8 = 0x00;
pub const MASS           : u8 = 0x01;
pub const DEBUG          : u8 = 0x02;
pub const SWIM           : u8 = 0x03;
pub const BOOTLOADER     : u8 = 0x04;
pub const UNKNOWN        : u8 = 0xFF;
