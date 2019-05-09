//! Enums used in the STLink


/// STLink JTAG version
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum STLinkJTAGVersion {
	V1 = 0b00010000,
	V2 = 0b00100000,
	V3 = 0b00110000,
}

impl std::fmt::Display for STLinkJTAGVersion {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}",match *self {
			STLinkJTAGVersion::V1 => "V1",
			STLinkJTAGVersion::V2 => "V2",
			STLinkJTAGVersion::V3 => "V3",
		})
	}
}


/// Direction of the data flow
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
	In,
	Out,
}

/// The current mode of the STLink
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum STLinkMode {
	Unknown,
	DFU,
	Mass,
	Debug(DebugMode),
}

impl Into<usize> for STLinkMode {
	fn into(self) -> usize {
		match self {
			STLinkMode::Unknown => 0,
			STLinkMode::DFU => 1,
			STLinkMode::Mass => 2,
			STLinkMode::Debug(d) => match d {
				DebugMode::JTAG => 3,
				DebugMode::SWD  => 4,
				DebugMode::SWIM => 5,
				_ => 0,
			},
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DebugMode {
	SWD,
	JTAG,
	SWIM,
	Unknown,
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlashType {
	TypeF4,
	TypeF0,
	TypeL0,
	TypeF1XL,
	TypeG0,
	TypeL4,
	TypeWB,
	Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cmd {
	Int8(u8),
	Int16(u16),
	Int32(u32),
}