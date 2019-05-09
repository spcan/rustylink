//! Link Register read and write

use crate::link::util::{ buf_write_u32, buf_read_u32 };

use super::super::structs::CoreRegisters;

use libusb::Direction;

use super::Link;

impl<'a> Link<'a> {
	/// Write to debug register
	pub fn write_debug_reg(&mut self, address: u32, value: u32) -> Result<(), ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND };

		// Match the command to the STLink version
		let writedebugreg = match self.version.jtag_api {
			1 => super::super::constants::commands::debug::apiv1::WRITEDEBUGREG,
			_ => super::super::constants::commands::debug::apiv2::WRITEDEBUGREG,
		};

		// Set up command
		self.cmd_setup(2, Direction::In);
		self.push_command(DEBUG_COMMAND);
		// Send command. Write Debug Register
		self.push_command(writedebugreg);
		// Send address
		buf_write_u32(&mut self.cmdbuf, self.cmdidx + 0, address, true);
		// Send new value
		buf_write_u32(&mut self.cmdbuf, self.cmdidx + 4, value,   true);
		self.cmdidx += 8;

		// Send command -> Receive nothing (ACK)
		match self.recv(self.cmdidx, 2, true) {
			Ok(_) => Ok(()),
			_ => {
				error!("Could not write to Debug Register at address: 0x{:X}", address);
				Err(())
			},
		}
	}

	/// Read a debug register
	pub fn read_debug_reg(&mut self, address: u32) -> Result<u32, ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, apiv2::READDEBUGREG };

		match self.version.stlink {
			// STLink V1 does not support this
			1 => Err(()),
			_ => {
				// Set up command
				self.cmd_setup(8, Direction::In);
				self.push_command(DEBUG_COMMAND);
				// Send command. Read Debug Register
				self.push_command(READDEBUGREG);

				// Send the address of the register
				buf_write_u32(&mut self.cmdbuf, self.cmdidx, address, true);
				self.cmdidx += 4;

				// Send command -> Receive the data
				match self.recv(self.cmdidx, 8, false) {
					Ok(_) => Ok(buf_read_u32(&self.databuf, 4, true)),
					_ => {
						error!("Could not read debug register.");
						Err(())
					},
				}
			},
		}
	}

	/// Read a register
	pub fn read_reg(&mut self, num: u8) -> Result<u32, ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND };

		let (readcommand, size) = match self.version.stlink {
			1 => (super::super::constants::commands::debug::apiv1::READREG, 4),
			_ => (super::super::constants::commands::debug::apiv2::READREG, 8),
		};

		self.cmd_setup(size as u32, Direction::In);
		self.push_command(DEBUG_COMMAND);
		self.push_command(readcommand);
		self.push_command(num);

		match self.recv(self.cmdidx, size, true) {
			Ok(_) => Ok( buf_read_u32(&self.databuf, size - 4, true) ),
			_ => {
				error!("Could not get core registers (not debug).");
				Err(())
			},
		}
	}
}


impl<'a> Link<'a> {
	/// Read the core registers (r0, r1, ...)
	pub fn read_core_regs(&mut self) -> Result<CoreRegisters, ()> {
		use super::super::constants::commands::debug::DEBUG_COMMAND;

		let (readcommand, nregs) = match self.version.jtag_api {
			1 => (super::super::constants::commands::debug::apiv1::READALLREGS, 84),
			_ => (super::super::constants::commands::debug::apiv2::READALLREGS, 88),
		};

		self.cmd_setup(nregs as u32, Direction::In);

		self.push_command(DEBUG_COMMAND);
		self.push_command(readcommand);

		match self.recv(self.cmdidx, nregs, true) {
			Ok(_) => {
				let mut regs = CoreRegisters::new();

				(0..=15).for_each(|i| regs.set_r(i, buf_read_u32(&self.databuf, i*4, true)));
				regs.set_xpsr(buf_read_u32(&self.databuf, 64, true));
				regs.set_msp( buf_read_u32(&self.databuf, 68, true));
				regs.set_psp( buf_read_u32(&self.databuf, 72, true));
				regs.set_rw(  buf_read_u32(&self.databuf, 76, true));
				regs.set_rw2( buf_read_u32(&self.databuf, 80, true));

				Ok( regs )
			},
			_ => {
				error!("Could not get core registers.");
				Err(())
			},
		}
	}
}