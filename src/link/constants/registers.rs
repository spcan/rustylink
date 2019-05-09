//! STLink registers

/// Debug Control Block
pub mod dcb {
	pub const DHCSREG : u32 = 0xE000EDF0;
	pub const DCRSREG : u32 = 0xE000EDF4;
	pub const DCRDREG : u32 = 0xE000EDF8;
	pub const DEMCREG : u32 = 0xE000EDFC;


	pub mod dhcsr {
		pub const DBGKEY		: u32 =  (0xA05F << 16);
		pub const C_DEBUGEN		: u32 =       (1 <<  0);
		pub const C_HALT		: u32 =       (1 <<  1);
		pub const C_STEP		: u32 =       (1 <<  2);
		pub const C_MASKINTS	: u32 =       (1 <<  3);
		pub const S_REGRDY		: u32 =       (1 << 16);
		pub const S_HALT		: u32 =       (1 << 17);
		pub const S_SLEEP		: u32 =       (1 << 18);
		pub const S_LOCKUP		: u32 =       (1 << 19);
		pub const S_RETIRE_ST	: u32 =       (1 << 24);
		pub const S_RESET_ST	: u32 =       (1 << 25);
	}


	pub mod demcr {
		pub const TRCENA		: u32 = (1 << 24);
		pub const VC_HARDERR	: u32 = (1 << 10);
		pub const VC_INTERR		: u32 = (1 <<  9);
		pub const VC_BUSERR		: u32 = (1 <<  8);
		pub const VC_STATERR	: u32 = (1 <<  7);
		pub const VC_CHKERR		: u32 = (1 <<  6);
		pub const VC_NOCPERR	: u32 = (1 <<  5);
		pub const VC_MMERR		: u32 = (1 <<  4);
		pub const VC_CORERESET	: u32 = (1 <<  0);
	}
}

pub mod nvic {
	pub mod register {
		pub const ICTR		: u32 = 0xE000E004;
		pub const ISE0		: u32 = 0xE000E100;
		pub const ICSR		: u32 = 0xE000ED04;
		pub const AIRCR		: u32 = 0xE000ED0C;
		pub const SHCSR		: u32 = 0xE000ED24;
		pub const CFSR		: u32 = 0xE000ED28;
		pub const MMFSRb	: u32 = 0xE000ED28;
		pub const BFSRb		: u32 = 0xE000ED29;
		pub const USFSRh	: u32 = 0xE000ED2A;
		pub const HFSR		: u32 = 0xE000ED2C;
		pub const DFSR		: u32 = 0xE000ED30;
		pub const MMFAR		: u32 = 0xE000ED34;
		pub const BFAR		: u32 = 0xE000ED38;
	}

	pub mod aircr {
		pub const VECTKEY		: u32 = (0x5FA << 16);
		pub const SYSRESETREQ	: u32 =      (1 << 2);
		pub const VECTCLRACTIVE	: u32 =      (1 << 1);
		pub const VECTRESET		: u32 =      (1 << 0);
	}

	pub mod shcsr {
		pub const BUSFAULTENA : u32 = (1 << 17);
	}

	pub mod dfsr {
		pub const HALTED  : u32 = 1;
		pub const BKPT    : u32 = 2;
		pub const DWTTRAP : u32 = 4;
		pub const VCATCH  : u32 = 8;
	}
}

pub mod itm {
	pub const TER0      : u32 = 0xE0000E00;
	pub const TPR       : u32 = 0xE0000E40;
	pub const TCR       : u32 = 0xE0000E80;
	pub const LAR       : u32 = 0xE0000FB0;
	pub const LAR_KEY   : u32 = 0xC5ACCE55;
}

pub mod dwt {
	pub const CTRL		: u32 = 0xE0001000;
	pub const CYCCNT	: u32 = 0xE0001004;
	pub const PCSR		: u32 = 0xE000101C;
	pub const COMP0		: u32 = 0xE0001020;
	pub const MASK0		: u32 = 0xE0001024;
	pub const FUNCTION0	: u32 = 0xE0001028;
}

pub mod fp {
	pub const CTRL	: u32 = 0xE0002000;
	pub const REMAP	: u32 = 0xE0002004;
	pub const COMP0	: u32 = 0xE0002008;
	pub const COMP1	: u32 = 0xE000200C;
	pub const COMP2	: u32 = 0xE0002010;
	pub const COMP3	: u32 = 0xE0002014;
	pub const COMP4	: u32 = 0xE0002018;
	pub const COMP5	: u32 = 0xE000201C;
	pub const COMP6	: u32 = 0xE0002020;
	pub const COMP7	: u32 = 0xE0002024;
}

pub mod fpu {
	pub const CPACR		: u32 = 0xE000ED88;
	pub const FPCCR		: u32 = 0xE000EF34;
	pub const FPCAR		: u32 = 0xE000EF38;
	pub const FPDSCR	: u32 = 0xE000EF3C;
}

pub mod tpiu {
	pub const SSPSR	: u32 = 0xE0040000;
	pub const CSPSR	: u32 = 0xE0040004;
	pub const ACPR	: u32 = 0xE0040010;
	pub const SPPR	: u32 = 0xE00400F0;
	pub const FFSR	: u32 = 0xE0040300;
	pub const FFCR	: u32 = 0xE0040304;
	pub const FSCR	: u32 = 0xE0040308;
}

pub mod fpcr {
	pub const CODE    : u32 = 0;
	pub const LITERAL : u32 = 1;
	pub const REPLACE_REMAP     : u32 = (0 << 30);
	pub const REPLACE_BKPT_LOW  : u32 = (1 << 30);
	pub const REPLACE_BKPT_HIGH : u32 = (2 << 30);
	pub const REPLACE_BKPT_BOTH : u32 = (3 << 30);
}