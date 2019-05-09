//! All methods for entering and exiting modes

use crate::link::enums::{ STLinkMode, DebugMode };
use super::Link;

use libusb::Direction;

use super::super::enums::Cmd;

impl<'a> Link<'a> {
	/// Enter SWD Mode
	pub fn enter_swd_mode(&mut self) -> Result<(), ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, ENTER_SWD_NO_RESET, apiv1::ENTER };

		match self.command(0, Direction::In, vec![Cmd::Int8(DEBUG_COMMAND), Cmd::Int8(ENTER), Cmd::Int8(ENTER_SWD_NO_RESET)]) {
			Ok(_) => {
				Ok(())
			}
			Err(_) => {
				error!("Enter SWD protocol. Reading failed.");
				Err(())
			},
		}
	}

	/// Exit DFU mode
	pub fn exit_dfu_mode(&mut self) -> Result<(), ()> {
		use super::super::constants::commands::dfu::{ COMMAND, EXIT };

		match self.command(0, Direction::In, vec![Cmd::Int8(COMMAND), Cmd::Int8(EXIT)]) {
			Ok(_) => {
				Ok(())
			}
			Err(_) => {
				error!("Exit DFU protocol. Writing failed.");
				Err(())
			},
		}
	}

	/// Exit debug mode
	pub fn exit_debug_mode(&mut self) -> Result<(), ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, EXIT };

		match self.command(0, Direction::In, vec![Cmd::Int8(DEBUG_COMMAND), Cmd::Int8(EXIT)]) {
			Ok(_) => {
				Ok(())
			}
			Err(_) => {
				error!("Exit Debug mode protocol. Writing failed.");
				Err(())
			},
		}
	}

	/// Leave the mode
	/// TODO : change and clean
	pub fn leave_mode(&mut self, mode: STLinkMode) -> Result<(), ()> {
		self.cmd_setup(2, Direction::In);

		match mode {
			STLinkMode::Debug(m) => match m {
				DebugMode::JTAG | DebugMode::SWD => {
					debug!("Leaving JTAG/SWD mode");
					self.push_command(super::super::constants::commands::debug::DEBUG_COMMAND);
					self.push_command(super::super::constants::commands::debug::EXIT);
				},
				DebugMode::SWIM => {
					debug!("Leaving SWIM mode");
					self.push_command(super::super::constants::commands::swim::SWIM_COMMAND);
					self.push_command(super::super::constants::commands::swim::EXIT);
				},
				n => {
					error!("Illegal Debug mode: {:?}", n);
					panic!();
				},
			},

			STLinkMode::DFU => {
				self.push_command(super::super::constants::commands::dfu::COMMAND);
				self.push_command(super::super::constants::commands::dfu::EXIT);
			}
			n => {
				error!("Illegal mode: {:?}", n);
				panic!();
			},
		}

		debug!("Sending leave mode command!");

		match self.recv(self.cmdidx, 0, true) {
			Ok(_) => Ok(()),
			_ => {
				error!("Leave mode protocol. Command error");
				Err(())
			},
		}
	}


	/// Enter mode
	/// TODO : change and clean
	pub fn enter_mode(&mut self, mode: DebugMode) -> Result<(), ()> {
		let mut rxsize = if self.version.jtag_api == 1 {
			0
		} else {
			2
		};

		self.cmd_setup(rxsize, Direction::In);
		match mode {
			DebugMode::JTAG => {
				self.push_command(super::super::constants::commands::debug::DEBUG_COMMAND);
				if self.version.jtag_api == 1 {
					self.push_command(super::super::constants::commands::debug::apiv1::ENTER);
				} else {
					self.push_command(super::super::constants::commands::debug::apiv2::ENTER);
				}
				self.push_command(super::super::constants::commands::debug::ENTER_JTAG_NO_RESET);
			},
			DebugMode::SWD => {
				self.push_command(super::super::constants::commands::debug::DEBUG_COMMAND);
				if self.version.jtag_api == 1 {
					self.push_command(super::super::constants::commands::debug::apiv1::ENTER);
				} else {
					self.push_command(super::super::constants::commands::debug::apiv2::ENTER);
				}
				self.push_command(super::super::constants::commands::debug::ENTER_SWD_NO_RESET);
			},
			DebugMode::SWIM => {
				self.push_command(super::super::constants::commands::swim::SWIM_COMMAND);
				self.push_command(super::super::constants::commands::swim::ENTER);
				// No response
				rxsize = 0;
			},
			_ => return Err(()),

		}

		match self.recv(self.cmdidx, rxsize as usize, true) {
			Ok(_) => {
				info!("Enter {:?} mode protocol. Correctly entered mode.", mode);
				Ok(())
			},
			_ => {
				error!("Enter mode {:?} protocol. Could not complete communication.", mode);
				print!("[", );
				for b in &self.databuf[0..16] {
					print!("  {}, ", b);
				}
				println!("]");
				print!("[", );
				for b in &self.databuf[0..16] {
					print!("{:#4X}, ", b);
				}
				println!("]");
				Err(())
			},
		}
	}

}