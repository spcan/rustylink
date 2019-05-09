//! Structures used in STLink

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Endpoints {
	rx: u8,
	tx: u8,
	trace: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct STLinkTrace {
	enabled: bool,
	source_hz: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpeedMap {
	pub speed: usize,
	pub divisor: usize,
}

impl SpeedMap {
	pub const fn new(speed: usize, divisor: usize) -> Self {
		Self { speed, divisor, }
	}
}

#[derive(Debug, Clone)]
struct JTAGXfer {
	ep: u8,
	buf: Vec<u8>,
	size: usize,
	/* Internal */
	retval: usize,
	completed: usize,
	transfer_size: usize,
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct STLinkUSBVersion {
	/** */
	pub stlink: usize,
	/** */
	pub jtag: usize,
	/** */
	pub swim: usize,
	/** jtag api version supported */
	pub jtag_api: usize,
	/** one bit for each feature supported. See macros STLINK_F_* */
	pub flags: u32,
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Endpoint {
	pub config:  u8,
	pub iface:   u8,
	pub setting: u8,
	pub address: u8,
}

/// Core Registers for the Cortex-M architecture
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CoreRegisters {
	inner: [u32; 22],
}

impl CoreRegisters {
	/// New empty `Self`
	pub fn new() -> Self {
		Self {
			inner: [0; 22],
		}
	}

	/// Set the R`i` register
	pub fn set_r(&mut self, i: usize, value: u32) {
		self.inner[i] = value;
	}

	/// Set the xPSR register
	pub fn set_xpsr(&mut self, value: u32) {
		self.inner[16] = value;
	}

	/// Set the MSP (main SP) register
	pub fn set_msp(&mut self, value: u32) {
		self.inner[17] = value;
	}

	/// Set the PSP (process SP) register
	pub fn set_psp(&mut self, value: u32) {
		self.inner[18] = value;
	}

	/// Set the RW register
	pub fn set_rw(&mut self, value: u32) {
		self.inner[19] = value;
	}

	/// Set the RW2 register
	pub fn set_rw2(&mut self, value: u32) {
		self.inner[20] = value;
	}
}

impl std::fmt::Display for CoreRegisters {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Registers:\n")?;
		(0..=15).map(|i| write!(f, "  r{}{} : {:#8X}\n", i, if i < 10 { " " } else { "" }, self.inner[i])).count();
		write!(f, "  xPSR: {:#8X}\n", self.inner[16])?;
		write!(f, "  mSP : {:#8X}\n", self.inner[17])?;
		write!(f, "  pSP : {:#8X}\n", self.inner[18])?;
		write!(f, "  RW1 : {:#8X}\n", self.inner[19])?;
		write!(f, "  RW2 : {:#8X}\n", self.inner[20])
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemInfo {
	pub flash: FlashInfo,
	pub ram: Vec<SRamInfo>,
	pub sys: SysMemInfo,
}

impl MemInfo {
	pub fn new() -> Self {
		Self{
			flash: FlashInfo { base: 0, size: 0, pagesize: None },
			ram: Vec::new(),
			sys: SysMemInfo { base: 0, size: 0 },
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FlashInfo {
	pub base: u32,
	pub size: u32,
	pub pagesize: Option<u32>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SRamInfo {
	pub base: u32,
	pub size: u32,
}

impl SRamInfo {
	pub const fn new(size: u32, base: u32) -> Self {
		Self { base, size, }
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SysMemInfo {
	pub base: u32,
	pub size: u32,
}