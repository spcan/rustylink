//! FLASH constants

pub mod register {
	pub const BASE    : u32 = 0x40023c00;
	pub const ACR     : u32 = 0x40023c00;
	pub const KEYR    : u32 = 0x40023c04;
	pub const OPTKEYR : u32 = 0x40023c08;
	pub const SR      : u32 = 0x40023c0C;
	pub const CR      : u32 = 0x40023c10;
	pub const OPTCR   : u32 = 0x40023c14;
	pub const OPTCR1  : u32 = 0x40023c18;
	pub const OPTCR2  : u32 = 0x40023c1c;
}

pub mod cr {
	pub const PG       : u32 = (1 <<  0);
	pub const SER      : u32 = (1 <<  1);
/// MER/MER1 for f76x/77x
	pub const MER      : u32 = (1 <<  2);
/// MER2 for f76x/77x, confusing ...
	pub const MER1     : u32 = (1 << 15);
	pub const STRT     : u32 = (1 << 16);
	pub const PSIZE_8  : u32 = (0 <<  8);
	pub const PSIZE_16 : u32 = (1 <<  8);
	pub const PSIZE_32 : u32 = (2 <<  8);
	pub const PSIZE_64 : u32 = (3 <<  8);
	pub const LOCK     : u32 = (1 << 31);
}

pub mod misc {
	/// Register unlock key 1
	pub const KEY1     : u32 = 0x45670123;
	/// Register unlock key 2
	pub const KEY2     : u32 = 0xCDEF89AB;

	/// Option register unlock key 1
	pub const OPTKEY1  : u32 = 0x08192A3B;
	/// Option register unlock key 2
	pub const OPTKEY2  : u32 = 0x4C5D6E7F;
}