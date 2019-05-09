//! Debug methods

use super::Link;

use super::super::enums::Cmd;

use libusb::Direction;

impl<'a> Link<'a> {
	/// Run
	pub fn run(&mut self) -> Result<(), ()> {
		use super::super::constants::registers::dcb::{ DHCSREG, dhcsr::{ DBGKEY, C_DEBUGEN } };
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, RUNCORE };

		match self.version.jtag_api {
			1 => {
				match self.command(2, Direction::In, vec![Cmd::Int8(DEBUG_COMMAND), Cmd::Int8(RUNCORE)]) {
					Ok(_) => Ok(()),
					_ => {
						error!("Failed to send run command to the device.");
						Err(())
					},
				}
			},
			_ => self.write_debug_reg(DHCSREG, DBGKEY|C_DEBUGEN),
		}
	}


	// Halt
	pub fn halt(&mut self) -> Result<(), ()> {
		use super::super::constants::registers::dcb::{ DHCSREG, dhcsr::{ DBGKEY, C_HALT, C_DEBUGEN } };
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, FORCEDEBUG };

		match self.version.jtag_api {
			// STLink V1
			// Write the Command Block register to halt and force debug
			1 => {
				// Send command -> Receive response
				match self.command(2, Direction::In, vec![Cmd::Int8(DEBUG_COMMAND), Cmd::Int8(FORCEDEBUG)]) {
					Ok(_) => Ok(()),
					_ => {
						error!("Could not halt device");
						Err(())
					},
				}
			},
			// Other STLink version
			_ => self.write_debug_reg(DHCSREG, DBGKEY|C_HALT|C_DEBUGEN),
		}
	}

	/// Step
	pub fn step(&mut self) -> Result<(), ()> {
		use super::super::constants::registers::dcb::{ DHCSREG, dhcsr::{ DBGKEY, C_HALT, C_DEBUGEN, C_MASKINTS, C_STEP } };
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, STEPCORE };

		match self.version.jtag_api {
			1 => {
				match self.command(2,Direction::In, vec![Cmd::Int8(DEBUG_COMMAND), Cmd::Int8(STEPCORE)]) {
					Ok(_) => Ok(()),
					_ => {
						error!("Could not send 'STEP' command");
						Err(())
					},
				}
			},
			_ => {
				self.write_debug_reg(DHCSREG, DBGKEY|C_HALT|C_MASKINTS|C_DEBUGEN)?;
				self.write_debug_reg(DHCSREG, DBGKEY|C_STEP|C_MASKINTS|C_DEBUGEN)?;
				self.write_debug_reg(DHCSREG, DBGKEY|C_HALT|C_DEBUGEN)
			},
		}
	}
}