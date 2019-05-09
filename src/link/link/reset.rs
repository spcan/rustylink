//! All reset methods

use super::Link;

use libusb::Direction;

use super::super::enums::{ Cmd, DebugMode, STLinkMode };

impl<'a> Link<'a> {
	/// USB Reset
	pub fn usb_reset(&mut self) -> Result<(), ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND };

		let resetsys = match self.version.jtag_api {
			1 => super::super::constants::commands::debug::apiv1::RESETSYS,
			_ => super::super::constants::commands::debug::apiv2::RESETSYS,
		};

		match self.command(2, Direction::In, vec![Cmd::Int8(DEBUG_COMMAND), Cmd::Int8(resetsys)]) {
			Ok(_) => Ok(()),
			Err(_) => {
				error!("Exit DFU protocol. Writing failed.");
				return Err(());
			},
		}

	}


	/// JTAG Reset
	pub fn jtag_reset(&mut self, value: u32) -> Result<(), ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, apiv2::DRIVE_NRST };

		match self.command(2, Direction::In, vec![Cmd::Int8(DEBUG_COMMAND), Cmd::Int8(DRIVE_NRST), Cmd::Int8(value as u8)]) {
			Ok(_) => {
				Ok(())
			}
			Err(_) => {
				error!("Exit DFU protocol. Writing failed.");
				Err(())
			},
		}
	}


	/// Assert Software Reset
	pub fn assert_srst(&mut self, mode: STLinkMode, srst: u8) -> Result<(), ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, apiv2::DRIVE_NRST };

		if mode == STLinkMode::Debug(DebugMode::SWIM) {
			return self.swim_assert_reset(srst);
		}

		if self.version.stlink == 1 {
			error!("STLink V1 cannot assert reset.");
			return Err(());
		}

		match self.command(2, Direction::In, vec![Cmd::Int8(DEBUG_COMMAND), Cmd::Int8(DRIVE_NRST), Cmd::Int8(srst)]) {
			Ok(_) => Ok(()),
			_ => {
				error!("Could not assert reset.");
				Err(())
			},
		}
	}

	/// Assert SWIM sreset line
	pub fn swim_assert_reset(&mut self, srst: u8) -> Result<(), ()> {
		use super::super::constants::commands::swim::{ SWIM_COMMAND, ASSERT_RESET, DEASSERT_RESET };

		let rstcmd = match srst {
			0 => ASSERT_RESET,
			_ => DEASSERT_RESET,
		};

		match self.command(0, Direction::In, vec![Cmd::Int8(SWIM_COMMAND), Cmd::Int8(rstcmd)]) {
			Ok(_) => Ok(()),
			Err(_) => {
				error!("Could not assert/deassert SWIM reset.");
				Err(())
			},
		}
	}
}