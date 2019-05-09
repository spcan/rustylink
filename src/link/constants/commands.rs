//! STLink commands module

pub mod swd {
	pub const AP_WAIT             : u8 = 0x10;
	pub const AP_FAULT            : u8 = 0x11;
	pub const AP_ERROR            : u8 = 0x12;
	pub const AP_PARITY_ERROR     : u8 = 0x13;
	pub const DP_WAIT             : u8 = 0x14;
	pub const DP_FAULT            : u8 = 0x15;
	pub const DP_ERROR            : u8 = 0x16;
	pub const DP_PARITY_ERROR     : u8 = 0x17;

	pub const AP_WDATA_ERROR      : u8 = 0x18;
	pub const AP_STICKY_ERROR     : u8 = 0x19;
	pub const AP_STICKYORUN_ERROR : u8 = 0x1a;
}

pub mod swim {
	pub const SWIM_COMMAND   : u8 = 0xF4;

	pub const ERR_OK         : u8 = 0x00;
	pub const BUSY           : u8 = 0x01;

	pub const ENTER          : u8 = 0x00;
	pub const EXIT           : u8 = 0x01;
	pub const READ_CAP       : u8 = 0x02;
	pub const SPEED          : u8 = 0x03;
	pub const ENTER_SEQ      : u8 = 0x04;
	pub const GEN_RST        : u8 = 0x05;
	pub const RESET          : u8 = 0x06;
	pub const ASSERT_RESET   : u8 = 0x07;
	pub const DEASSERT_RESET : u8 = 0x08;
	pub const READSTATUS     : u8 = 0x09;
	pub const WRITEMEM       : u8 = 0x0a;
	pub const READMEM        : u8 = 0x0b;
	pub const READBUF        : u8 = 0x0c;
}

pub mod debug {
	pub const DEBUG_COMMAND       : u8 = 0xF2;

	pub mod apiv1 {
		pub const SETFP           : u8 = 0x0b;
		pub const CLEARFP         : u8 = 0x0e;
		pub const WRITEDEBUGREG   : u8 = 0x0f;
		pub const SETWATCHPOINT   : u8 = 0x10;
		pub const RESETSYS        : u8 = 0x03;
		pub const READALLREGS     : u8 = 0x04;
		pub const READREG         : u8 = 0x05;
		pub const WRITEREG        : u8 = 0x06;
		pub const ENTER           : u8 = 0x20;
	}

	pub mod apiv2 {
		pub const ENTER           : u8 = 0x30;
		pub const READ_IDCODES    : u8 = 0x31;
		pub const RESETSYS        : u8 = 0x32;
		pub const READREG         : u8 = 0x33;
		pub const WRITEREG        : u8 = 0x34;
		pub const WRITEDEBUGREG   : u8 = 0x35;
		pub const READDEBUGREG    : u8 = 0x36;

		pub const READALLREGS     : u8 = 0x3A;
		pub const GETLASTRWSTATUS : u8 = 0x3B;
		pub const DRIVE_NRST      : u8 = 0x3C;

		pub const GETLASTRWSTATUS2 : u8 = 0x3E;

		pub const START_TRACE_RX  : u8 = 0x40;
		pub const STOP_TRACE_RX   : u8 = 0x41;
		pub const GET_TRACE_NB    : u8 = 0x42;
		pub const SWD_SET_FREQ    : u8 = 0x43;
		pub const JTAG_SET_FREQ   : u8 = 0x44;

		pub const READMEM_16BIT   : u8 = 0x47;
		pub const WRITEMEM_16BIT  : u8 = 0x48;

		pub const DRIVE_NRST_LOW   : u8 = 0x00;
		pub const DRIVE_NRST_HIGH  : u8 = 0x01;
		pub const DRIVE_NRST_PULSE : u8 = 0x02;
	}

	pub mod apiv3 {
		pub const SET_COM_FREQ : u8 = 0x61;
		pub const GET_COM_FREQ : u8 = 0x62;
	}

	pub const GETSTATUS             : u8 = 0x01;
	pub const FORCEDEBUG            : u8 = 0x02;
	pub const READMEM_32BIT         : u8 = 0x07;
	pub const WRITEMEM_32BIT        : u8 = 0x08;
	pub const RUNCORE               : u8 = 0x09;
	pub const STEPCORE              : u8 = 0x0a;
	pub const READMEM_8BIT          : u8 = 0x0c;
	pub const WRITEMEM_8BIT         : u8 = 0x0d;

	pub const ENTER_JTAG_RESET      : u8 = 0x00;
	pub const ENTER_SWD_NO_RESET    : u8 = 0xa3;
	pub const ENTER_JTAG_NO_RESET   : u8 = 0xa4;

	pub const EXIT                  : u8 = 0x21;
	pub const READCOREID            : u8 = 0x22;

	pub const ERR_OK            : u8 = 0x80;
	pub const ERR_FAULT         : u8 = 0x81;
}

pub mod dfu {
	pub const EXIT    : u8 = 0x07;
	pub const COMMAND : u8 = 0xF3;
}

pub const STLINK_BAD_AP_ERROR            : u32 = 0x1d;

pub const STLINK_CORE_RUNNING            : u32 = 0x80;
pub const STLINK_CORE_HALTED             : u32 = 0x81;
pub const STLINK_CORE_STAT_UNKNOWN       : u32 = 0xFFFFFFFF;

pub const GET_VERSION             : u8 = 0xF1;
pub const GET_CURRENT_MODE        : u8 = 0xF5;
pub const GET_TARGET_VOLTAGE      : u8 = 0xF7;


