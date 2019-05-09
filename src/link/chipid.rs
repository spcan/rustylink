//! STM chip id

use super::enums::FlashType;

use super::structs::SRamInfo;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum STM32ChipID {
	Unknown = 0x000,

	F1Medium      = 0x410,
	F2            = 0x411,
	F1Low         = 0x412,
	F4            = 0x413,
	F1High        = 0x414,
	L4            = 0x415,
	L1Medium      = 0x416,
	L0            = 0x417,
	F1Conn        = 0x418,
	F4HD          = 0x419,
	F1VLMediumLow = 0x420,
	F446          = 0x421,
	F3            = 0x422,
	F4LP          = 0x423,
	L0Cat2        = 0x425,
	L1MediumPlus  = 0x427,
	F1VLHigh      = 0x428,
	L1Cat2        = 0x429,
	F1XL          = 0x430,
	F411RE        = 0x431,
	F37x          = 0x432,
	F4DE          = 0x433,
	F4DSI         = 0x434,

	/// 0x435 covers STM32L43xxx and STM32L44xxx devices
	L43X             = 0x435,
	/// 0x461 covers STM32L496xx and STM32L4A6xx devices
	L496X            = 0x461,
	/// 0x462 covers STM32L45xxx and STM32L46xxx devices
	L46X             = 0x462,
	/// 0x464 covers STM32L41xxx and STM32L42xxx devices
	L41X             = 0x464,

	/// 0x436 is actually assigned to some L1 chips that are called "Medium-Plus"
	/// and some that are called "High".  0x427 is assigned to the other "Medium-
	/// plus" chips.  To make it a bit simpler we just call 427 MEDIUM_PLUS and
	/// 0x436 HIGH.
	L1High   = 0x436,
	L152RE   = 0x437,
	F334     = 0x438,
	F3Small  = 0x439,
	F0       = 0x440,
	F412     = 0x441,
	F09X     = 0x442,
	F0Small  = 0x444,
	F04      = 0x445,
	F303High = 0x446,
	L0Cat5   = 0x447,
	F0Can    = 0x448,
	///This ID is found on the NucleoF746ZG board
	F7       = 0x449,
	F7XXXX   = 0x451,
	///This ID is found on the NucleoF722ZE board
	F72XXX   = 0x452,
	L011     = 0x457,
	F410     = 0x458,
	F413     = 0x463,
	/// STM32L4R9I-DISCO board
	L4RX     = 0x470,
	G0X1     = 0x460,
	WB55     = 0x495,
}


#[derive(Debug, Clone)]
pub struct ChipParams {
	pub id: u32,
	pub description: String,
	pub flasht: FlashType,
	pub flash_size_reg: u32,
	pub pagesize: u32,
	pub sram: Vec<SRamInfo>,
	pub bootrom_base: u32,
	pub bootrom_size: u32,
}


lazy_static!{
	pub static ref STMCHIPS: [ChipParams; 48] = [
		ChipParams {
			id: STM32ChipID::F7XXXX as u32,
			description: String::from("F76xxx device"),
			flasht: FlashType::TypeF4,
			flash_size_reg: 0x1FF0_F442,
			pagesize: 0x800,                                  // 2 kB
			sram: vec![SRamInfo::new(0x80000, 0x2000_0000)],        // 512 kB
			bootrom_base: 0x0020_0000,                              // "System memory" start address
			bootrom_size: 0xEDC0,                                   // 59 kB + 448 bytes
		},

		ChipParams {
			id: STM32ChipID::F7 as u32,
			description: String::from("F7 device"),
			flasht: FlashType::TypeF4,
			flash_size_reg: 0x1FF0_F442,
			pagesize: 0x800,      // 2 kB
			sram: vec![SRamInfo::new(0x50000, 0x2000_0000)],        // 320 kB
			bootrom_base: 0x0010_0000,  // "System memory" start address
			bootrom_size: 0xEDC0,       // 59 kB + 448 bytes
		},

		ChipParams {
			id: STM32ChipID::F72XXX as u32,
			description: "F72 device".to_owned(),
			flasht: FlashType::TypeF4,
			flash_size_reg: 0x1FF0_F442,
			pagesize: 0x800,      // 2 kB
			sram: vec![SRamInfo::new(0x40000, 0x2000_0000)],        // 256 kB
			bootrom_base: 0x0010_0000,  // "System memory" start address
			bootrom_size: 0xEDC0,       // 59 kB + 448 bytes
		},


		ChipParams {
			id: STM32ChipID::F1Medium as u32,
			description: "F1 medium-density device".to_owned(),
			flasht: FlashType::TypeF0,
			flash_size_reg: 0x1FFF_F7E0,
			pagesize: 0x400,      // 1 kB
			sram: vec![SRamInfo::new(0x5000, 0x2000_0000)],         // 20 kB
			bootrom_base: 0x1FFF_F000,  // "System memory" start address
			bootrom_size: 0x800,        // 2 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F2 as u32,
			description: "F2 device".to_owned(),
			flasht: FlashType::TypeF4,
			flash_size_reg: 0x1FFF_7A22,
			pagesize: 0x20000,      // No flash page
			sram: vec![SRamInfo::new(0x20000, 0x2000_0000)],          // 128 kB
			bootrom_base: 0x1FFF_0000,    // "System memory" start address
			bootrom_size: 0x7800,         // 30 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F1Low as u32,
			description: "F1 low-density device".to_owned(),
			flasht: FlashType::TypeF0,
			flash_size_reg: 0x1FFF_F7E0,
			pagesize: 0x400,      // 1 kB
			sram: vec![SRamInfo::new(0x2800, 0x2000_0000)],         // 10 kB
			bootrom_base: 0x1FFF_F000,  // "System memory" start address
			bootrom_size: 0x800,        // 2 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F4 as u32,
			description: "F4 device".to_owned(),
			flasht: FlashType::TypeF4,
			flash_size_reg: 0x1FFF_7A22,
			pagesize: 0x4000,      // 16 kB
			sram: vec![SRamInfo::new(0x30000, 0x2000_0000)],         // 192 kB
			bootrom_base: 0x1FFF_0000,   // "System memory" start address
			bootrom_size: 0x7800,        // 30 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F4DSI as u32,
			description: "F46x and F47x device".to_owned(),
			flasht: FlashType::TypeF4,
			flash_size_reg: 0x1FFF_7A22,
			pagesize: 0x4000,      // 16 kB
			sram: vec![SRamInfo::new(0x40000, 0x2000_0000)],         // 256 kB
			bootrom_base: 0x1FFF_0000,   // "System memory" start address
			bootrom_size: 0x7800,        // 30 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F4HD as u32,
			description: "F42x and F43x device".to_owned(),
			flasht: FlashType::TypeF4,
			flash_size_reg: 0x1FFF_7A22,
			pagesize: 0x4000,      // 16 kB
			sram: vec![SRamInfo::new(0x40000, 0x2000_0000)],         // 256 kB
			bootrom_base: 0x1FFF_0000,   // "System memory" start address
			bootrom_size: 0x7800,        // 30 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F4LP as u32,
			description: "F4 device (low power)".to_owned(),
			flasht: FlashType::TypeF4,
			flash_size_reg: 0x1FFF_7A22,
			pagesize: 0x4000,      // 16 kB
			sram: vec![SRamInfo::new(0x10000, 0x2000_0000)],         // 64 kB
			bootrom_base: 0x1FFF_0000,   // "System memory" start address
			bootrom_size: 0x7800,        // 30 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F411RE as u32,
			description: "F4 device (low power) - stm32f411re".to_owned(),
			flasht: FlashType::TypeF4,
			flash_size_reg: 0x1FFF_7A22,
			pagesize: 0x4000,      // 16 kB
			sram: vec![SRamInfo::new(0x20000, 0x2000_0000)],         // 128 kB
			bootrom_base: 0x1FFF_0000,   // "System memory" start address
			bootrom_size: 0x7800,        // 30 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F4DE as u32,
			description: "F4 device (dynamic efficiency)".to_owned(),
			flasht: FlashType::TypeF4,
			flash_size_reg: 0x1FFF_7A22,
			pagesize: 0x4000,      // 16 kB
			sram: vec![SRamInfo::new(0x18000, 0x2000_0000)],         // 96 kB
			bootrom_base: 0x1FFF_0000,   // "System memory" start address
			bootrom_size: 0x7800,        // 30 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F1High as u32,
			description: "F1 high-density device".to_owned(),
			flasht: FlashType::TypeF0,
			flash_size_reg: 0x1FFF_F7E0,
			pagesize: 0x800,      // 2 kB
			sram: vec![SRamInfo::new(0x10000, 0x2000_0000)],        // 64 kB
			bootrom_base: 0x1FFF_0000,  // "System memory" start address
			bootrom_size: 0x800,        // 2 kB
		},

		// TODO
		// This ignores the EEPROM
		// This uses the page erase size, not the sector write protection
		ChipParams {
			id: STM32ChipID::L1Medium as u32,
			description: "L1 medium-density device".to_owned(),
			flasht: FlashType::TypeL0,
			flash_size_reg: 0x1FF8_004C,
			pagesize: 0x100,      // 256 B
			sram: vec![SRamInfo::new(0x4000, 0x2000_0000)],         // 16 kB
			bootrom_base: 0x1FF0_0000,  // "System memory" start address
			bootrom_size: 0x1000,       // 4 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::L1Cat2 as u32,
			description: "L1 Cat.2 device".to_owned(),
			flasht: FlashType::TypeL0,
			flash_size_reg: 0x1FF8_004C,
			pagesize: 0x100,      // 256 B
			sram: vec![SRamInfo::new(0x8000, 0x2000_0000)],         // 32 kB
			bootrom_base: 0x1FF0_0000,  // "System memory" start address
			bootrom_size: 0x1000,       // 4 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::L1MediumPlus as u32,
			description: "L1 medium-plus-density device".to_owned(),
			flasht: FlashType::TypeL0,
			flash_size_reg: 0x1FF8_00CC,
			pagesize: 0x100,      // 256 B
			// Maybe some with 48K
			sram: vec![SRamInfo::new(0x8000, 0x2000_0000)],         // 32 kB
			bootrom_base: 0x1FF0_0000,  // "System memory" start address
			bootrom_size: 0x1000,       // 4 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::L1High as u32,
			description: "L1 high-density device".to_owned(),
			flasht: FlashType::TypeL0,
			flash_size_reg: 0x1FF8_00CC,
			pagesize: 0x100,      // 256 B
			sram: vec![SRamInfo::new(0xC000, 0x2000_0000)],         // 48 kB
			bootrom_base: 0x1FF0_0000,  // "System memory" start address
			bootrom_size: 0x1000,       // 4 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::L152RE as u32,
			description: "L152RE device".to_owned(),
			flasht: FlashType::TypeL0,
			flash_size_reg: 0x1FF8_00CC,
			pagesize: 0x100,      // 256 B
			sram: vec![SRamInfo::new(0x14000, 0x2000_0000)],        // 80 kB
			bootrom_base: 0x1FF0_0000,  // "System memory" start address
			bootrom_size: 0x1000,       // 4 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F1Conn as u32,
			description: "F1 Connectivity line device".to_owned(),
			flasht: FlashType::TypeF0,
			flash_size_reg: 0x1FFF_F7E0,
			pagesize: 0x800,      // 2 kB
			sram: vec![SRamInfo::new(0x10000, 0x2000_0000)],        // 64 kB
			bootrom_base: 0x1FFF_B000,  // "System memory" start address
			bootrom_size: 0x4800,       // 18 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F1VLMediumLow as u32,
			description: "F1 medium/low-density Value Line device".to_owned(),
			flasht: FlashType::TypeF0,
			flash_size_reg: 0x1FFF_F7E0,
			pagesize: 0x400,      // 1 kB
			sram: vec![SRamInfo::new(0x2000, 0x2000_0000)],         // 8 kB
			bootrom_base: 0x1FFF_F000,  // "System memory" start address
			bootrom_size: 0x800,        // 2 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F446 as u32,
			description: "F446 device".to_owned(),
			flasht: FlashType::TypeF4,
			flash_size_reg: 0x1FFF_7A22,
			pagesize: 0x20000,      // No flash page
			sram: vec![SRamInfo::new(0x20000, 0x2000_0000)],          // 128 kB
			bootrom_base: 0x1FFF_0000,    // "System memory" start address
			bootrom_size: 0x7800,         // 30 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F410 as u32,
			description: "F410 device".to_owned(),
			flasht: FlashType::TypeF4,
			flash_size_reg: 0x1FFF_7A22,
			pagesize: 0x4000,      // 16 kB
			sram: vec![SRamInfo::new(0x8000, 0x2000_0000)],          // 32 kB
			bootrom_base: 0x1FFF_0000,   // "System memory" start address
			bootrom_size: 0x7800,        // 30 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F3 as u32,
			description: "F3 device".to_owned(),
			flasht: FlashType::TypeF0,
			flash_size_reg: 0x1FFF_F7CC,
			pagesize: 0x800,      // 2 kB
			sram: vec![SRamInfo::new(0xA000, 0x2000_0000)],         // 40 kB
			bootrom_base: 0x1FFF_F000,  // "System memory" start address
			bootrom_size: 0x800,        // 2 kB
		},

		// TODO
		// This is STK32F373VCT6 device from STM32 F373 eval board
		// Support based on 303 above (37x and 30x have same memory map)
		ChipParams {
			id: STM32ChipID::F37x as u32,
			description: "F37x device".to_owned(),
			flasht: FlashType::TypeF0,
			flash_size_reg: 0x1FFF_F7CC,
			pagesize: 0x800,      // 2 kB
			sram: vec![SRamInfo::new(0xA000, 0x2000_0000)],         // 40 kB
			bootrom_base: 0x1FFF_F000,  // "System memory" start address
			bootrom_size: 0x800,        // 2 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F1VLHigh as u32,
			description: "F1 high-density value line device".to_owned(),
			flasht: FlashType::TypeF0,
			flash_size_reg: 0x1FFF_F7E0,
			pagesize: 0x800,      // 2 kB
			sram: vec![SRamInfo::new(0x8000, 0x2000_0000)],         // 32 kB
			bootrom_base: 0x1FFF_F000,  // "System memory" start address
			bootrom_size: 0x800,        // 2 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F1XL as u32,
			description: "F1 XL-density device".to_owned(),
			flasht: FlashType::TypeF1XL,
			flash_size_reg: 0x1FFF_F7E0,
			pagesize: 0x800,      // 2 kB
			sram: vec![SRamInfo::new(0x18000, 0x2000_0000)],         // 96 kB
			bootrom_base: 0x1FFF_E000,  // "System memory" start address
			bootrom_size: 0x1800,       // 6 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F0Can as u32,
			description: "F07x device".to_owned(),
			flasht: FlashType::TypeF0,
			flash_size_reg: 0x1FFF_F7CC,
			pagesize: 0x800,      // 2 kB
			sram: vec![SRamInfo::new(0x4000, 0x2000_0000)],         // 16 kB
			bootrom_base: 0x1FFF_C800,  // "System memory" start address
			bootrom_size: 0x3000,       // 12 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F0 as u32,
			description: "F0 device".to_owned(),
			flasht: FlashType::TypeF0,
			flash_size_reg: 0x1FFF_F7CC,
			pagesize: 0x400,      // 1 kB
			sram: vec![SRamInfo::new(0x2000, 0x2000_0000)],         // 8 kB
			bootrom_base: 0x1FFF_EC00,  // "System memory" start address
			bootrom_size: 0xC00,        // 3 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F412 as u32,
			description: "F412 device".to_owned(),
			flasht: FlashType::TypeF4,
			flash_size_reg: 0x1FFF_7A22,
			pagesize: 0x4000,      // 16 kB
			sram: vec![SRamInfo::new(0x40000, 0x2000_0000)],         // 256 kB
			bootrom_base: 0x1FFF_0000,   // "System memory" start address
			bootrom_size: 0x7800,        // 30 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F413 as u32,
			description: "F413 device".to_owned(),
			flasht: FlashType::TypeF4,
			flash_size_reg: 0x1FFF_7A22,
			pagesize: 0x4000,      // 16 kB
			sram: vec![SRamInfo::new(0x50000, 0x2000_0000)],         // 320 kB
			bootrom_base: 0x1FFF_0000,   // "System memory" start address
			bootrom_size: 0x7800,        // 30 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F09X as u32,
			description: "F09x device".to_owned(),
			flasht: FlashType::TypeF0,
			flash_size_reg: 0x1FFF_F7CC,
			pagesize: 0x800,      // 2 kB
			sram: vec![SRamInfo::new(0x8000, 0x2000_0000)],         // 32 kB
			bootrom_base: 0x1FFF_D800,  // "System memory" start address
			bootrom_size: 0x2000,       // 8 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F04 as u32,
			description: "F04x device".to_owned(),
			flasht: FlashType::TypeF0,
			flash_size_reg: 0x1FFF_F7CC,
			pagesize: 0x400,      // 1 kB
			sram: vec![SRamInfo::new(0x1800, 0x2000_0000)],         // 6 kB
			bootrom_base: 0x1FFF_EC00,  // "System memory" start address
			bootrom_size: 0xC00,        // 3 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F0Small as u32,
			description: "F0 small device".to_owned(),
			flasht: FlashType::TypeF0,
			flash_size_reg: 0x1FFF_F7CC,
			pagesize: 0x400,      // 1 kB
			sram: vec![SRamInfo::new(0x1000, 0x2000_0000)],         // 4 kB
			bootrom_base: 0x1FFF_EC00,  // "System memory" start address
			bootrom_size: 0xC00,        // 3 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F3Small as u32,
			description: "F3 small device".to_owned(),
			flasht: FlashType::TypeF0,
			flash_size_reg: 0x1FFF_F7CC,
			pagesize: 0x800,      // 2 kB
			sram: vec![SRamInfo::new(0xA000, 0x2000_0000)],         // 40 kB
			bootrom_base: 0x1FFF_D800,  // "System memory" start address
			bootrom_size: 0x2000,       // 8 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::L0 as u32,
			description: "L0x3 device".to_owned(),
			flasht: FlashType::TypeL0,
			flash_size_reg: 0x1FF8_007C,
			pagesize: 0x80,       // 128 B
			sram: vec![SRamInfo::new(0x2000, 0x2000_0000)],         // 8 kB
			bootrom_base: 0x1FF_0000,   // "System memory" start address
			bootrom_size: 0x1000,       // 4 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::L0Cat5 as u32,
			description: "L0x Category 5 device".to_owned(),
			flasht: FlashType::TypeL0,
			flash_size_reg: 0x1FF8_007C,
			pagesize: 0x80,      // 128 B
			sram: vec![SRamInfo::new(0x5000, 0x2000_0000)],         // 20 kB
			bootrom_base: 0x1FF_0000,  // "System memory" start address
			bootrom_size: 0x2000,      // 8 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::L0Cat2 as u32,
			description: "L0x Category 2 device".to_owned(),
			flasht: FlashType::TypeL0,
			flash_size_reg: 0x1FF8_007C,
			pagesize: 0x80,       // 128 B
			sram: vec![SRamInfo::new(0x2000, 0x2000_0000)],         // 8 kB
			bootrom_base: 0x1FF_0000,   // "System memory" start address
			bootrom_size: 0x1000,       // 4 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F334 as u32,
			description: "F3xx medium density device".to_owned(),
			flasht: FlashType::TypeF0,
			flash_size_reg: 0x1FFF_F7CC,
			pagesize: 0x800,      // 2 kB
			sram: vec![SRamInfo::new(0x3000, 0x2000_0000)],         // 12 kB
			bootrom_base: 0x1FFF_D800,  // "System memory" start address
			bootrom_size: 0x2000,       // 8 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::F303High as u32,
			description: "L303 high density device".to_owned(),
			flasht: FlashType::TypeF0,
			flash_size_reg: 0x1FFF_F7CC,
			pagesize: 0x800,      // 2 kB
			sram: vec![SRamInfo::new(0x10000, 0x2000_0000)],        // 64 kB
			bootrom_base: 0x1FFF_D800,  // "System memory" start address
			bootrom_size: 0x2000,       // 8 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::L4 as u32,
			description: "L4 device".to_owned(),
			flasht: FlashType::TypeL4,
			flash_size_reg: 0x1FFF_75E0,
			pagesize: 0x800,      // 2 kB
			// SRAM1 is "up to" 96k in the standard memory map
			// SRAM2 is 32k mapped at 0x1000_0000
			sram: vec![SRamInfo::new(0x18000, 0x2000_0000),SRamInfo::new(0x8000, 0x1000_0000)],// 96 kB + 32 kB
			bootrom_base: 0x1FFF_0000,  // "System memory" start address
			bootrom_size: 0x7000,       // 28 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::L4RX as u32,
			description: "L4Rx device".to_owned(),
			flasht: FlashType::TypeL4,
			flash_size_reg: 0x1FFF_75E0,
			pagesize: 0x1000,      // 4 kB
			sram: vec![SRamInfo::new(0xA0000, 0x2000_0000)],         // 40 kB
			bootrom_base: 0x1FFF_0000,   // "System memory" start address
			bootrom_size: 0x7000,        // 28 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::L43X as u32,
			description: "L43x/L44x device".to_owned(),
			flasht: FlashType::TypeL4,
			flash_size_reg: 0x1FFF_75E0,
			pagesize: 0x800,      // 2 kB
			// SRAM1 is "up to" 64k in the standard memory map
			// SRAM2 is 16k mapped at 1000_0000
			sram: vec![SRamInfo::new(0xC000, 0x2000_0000), SRamInfo::new(0x4000, 0x1000_0000)], // 48 kB + 16 kB
			bootrom_base: 0x1FFF_0000,  // "System memory" start address
			bootrom_size: 0x7000,       // 28 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::L496X as u32,
			description: "L496x/L4A6x device".to_owned(),
			flasht: FlashType::TypeL4,
			flash_size_reg: 0x1FFF_75E0,
			pagesize: 0x800,       // 2 kB
			// SRAM1 is 256k at 0x2000_0000
			// SRAM2 is 64k at 0x2004_0000
			sram: vec![SRamInfo::new(0x40000, 0x2000_0000), SRamInfo::new(0x10000, 0x1000_0000)],// 256 kB + 64 kB
			bootrom_base: 0x1FFF_0000,   // "System memory" start address
			bootrom_size: 0x7000,        // 28 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::L46X as u32,
			description: "L45x/L46x device".to_owned(),
			flasht: FlashType::TypeL4,
			flash_size_reg: 0x1FFF_75E0,
			pagesize: 0x800,      // 2 kB
			// SRAM1 is 128k at 0x2000_0000
			// SRAM2 is 32k at 0x1000_0000
			sram: vec![SRamInfo::new(0x20000, 0x2000_0000), SRamInfo::new(0x8000, 0x1000_0000)],// 128 kB + 32 kB
			bootrom_base: 0x1FFF_0000,  // "System memory" start address
			bootrom_size: 0x7000,       // 28 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::L011 as u32,
			description: "L011 device".to_owned(),
			flasht: FlashType::TypeL0,
			flash_size_reg: 0x1FF8_007C,
			pagesize: 0x80,       // 128 B
			sram: vec![SRamInfo::new(0x2000, 0x2000_0000)],         // 8 kB
			bootrom_base: 0x1FF0_0000,  // "System memory" start address
			bootrom_size: 0x2000,       // 8 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::G0X1 as u32,
			description: "G071/G081 device".to_owned(),
			flasht: FlashType::TypeG0,
			flash_size_reg: 0x1FFF_75E0,
			pagesize: 0x800,      // 2 kB
			sram: vec![SRamInfo::new(0x9000, 0x2000_0000)],         // 36 kB
			bootrom_base: 0x1FFF_0000,  // "System memory" start address
			bootrom_size: 0x7800,       // 30 kB
		},

		// TODO
		ChipParams {
			id: STM32ChipID::WB55 as u32,
			description: "WB55 device".to_owned(),
			flasht: FlashType::TypeWB,
			flash_size_reg: 0x1FFF75E0,
			pagesize: 0x1000,      // 4 kB
			sram: vec![SRamInfo::new(0x40000, 0x2000_0000)],         // 256 kB
			bootrom_base: 0x1FFF_0000,   // "System memory" start address
			bootrom_size: 0x7000,        // 28 kB
		},

		// Unknown
		ChipParams {
			id: STM32ChipID::Unknown as u32,
			description: "Unknown device".to_owned(),
			flasht: FlashType::Unknown,
			flash_size_reg: 0x0,
			pagesize: 0x0,
			sram: vec![SRamInfo::new(0x0, 0x0)],
			bootrom_base: 0x0,
			bootrom_size: 0x0,
		},

	];
}


/// Allows to get a STM32 chip from its ID
pub fn get_chip_from_id_u32(id: u32) -> ChipParams {
	match STMCHIPS.iter().find(|chip| chip.id == id ) {
		Some(chip) => chip.clone(),
		_ => match STMCHIPS.last() {
			Some(chip) => chip.clone(),
			_ => panic!("Corrupted program data. - STMChip is a `static` but the last element wasn't found")
		},
	}
}