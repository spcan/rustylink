//! Misc



pub mod TIMEOUT {
	pub const WRITE : std::time::Duration = std::time::Duration::from_secs(120);
	pub const READ  : std::time::Duration = std::time::Duration::from_secs(120);
}


pub mod SIZE {
	pub const SG        : usize = (31);
	pub const DATA      : usize = (4096);
}

pub mod ENDPOINT {
	pub const ENDPOINT_IN  : u8 = 0x80;
	pub const ENDPOINT_OUT : u8 = 0x00;

	pub const STLINK_NULL_EP        : u8 = 0;
	pub const STLINK_RX_EP          : u8 = (1|ENDPOINT_IN);
	pub const STLINK_TX_EP          : u8 = (2|ENDPOINT_OUT);
	pub const STLINK_TRACE_EP       : u8 = (3|ENDPOINT_IN);
}

pub mod V1 {
	pub const PID           : u16 = (0x3744);
	pub const CMD_SIZE      : usize = (10);
	pub const MAX_RW   : usize =   64;
}

pub mod V2 {
	pub const PID           : u16 = (0x3748);
	pub const CMD_SIZE      : usize = (16);
	pub const MAX_RW   : usize =   64;
}

pub mod V2_1 {
	pub const STLINK_V2_1_TX_EP     : u8 = (1|super::ENDPOINT::ENDPOINT_OUT);
	pub const STLINK_V2_1_TRACE_EP  : u8 = (2|super::ENDPOINT::ENDPOINT_IN);

	pub const STLINK_V2_1_PID         : u16 = (0x374B);
	pub const STLINK_V2_1_NO_MSD_PID  : u16 = (0x3752);
	pub const MAX_RW   : usize =   64;
}

pub mod V3 {
	pub const USBLOADER_PID : u16 = (0x374D);
	pub const MAX_RW : usize =  512;
	pub const MAX_FREQ_NB : u8 = 10;
}

pub mod V3_2 {
	pub const PID : u16 = (0x3753);
	pub const MAX_RW : usize =  512;
}

pub mod V3E {
	pub const PID : u16 = (0x374E);
	pub const MAX_RW : usize =  512;
}

pub mod V3S {
	pub const PID : u16 = (0x374F);
	pub const MAX_RW : usize =  512;
}