//! Flags used in STLink

pub const HAS_TRACE              : u32 = 1 << 0;
pub const HAS_SWD_SET_FREQ       : u32 = 1 << 1;
pub const HAS_JTAG_SET_FREQ      : u32 = 1 << 2;
pub const HAS_MEM_16BIT          : u32 = 1 << 3;
pub const HAS_GETLASTRWSTATUS2   : u32 = 1 << 4;

/* aliases */
pub const HAS_TARGET_VOLT : u32 = HAS_TRACE;
