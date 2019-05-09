//! Flash interaction

use crate::link::util::{ buf_write_u32, buf_read_u32, buf_write_u16 };

use libusb::Direction;

use super::Link;

impl<'a> Link<'a> {
	/// Read `n` bytes from an `address` in memory
	/// The read will be done in sequential mode, 8 bytes at a time.
	pub fn read_mem(&mut self, address: u32, n: usize) -> Result<Vec<u8>, ()> {
		let transfers = n as u32 / 8;

		let mut out = Vec::new();
		let mut corrupted = false;

		(0..transfers).for_each(|i| {
			match self.read_mem8(address + i*8, 8) {
				Ok(_) => out.extend_from_slice(&self.databuf[0..8]),
				_ => {
					error!("General Read protocol. Failure to read memory in main sequence. Corrupted data.");
					corrupted = true;
				},
			}
		});

		if corrupted { return Err(()); }

		match n % 8 {
			0 => Ok( out ),
			b => {
				match self.read_mem8(address + transfers*8, b) {
					Ok(_) => {
						out.extend_from_slice(&self.databuf[0..b]);
						Ok( out )
					},
					_ => {
						error!("General Read protocol. Failure to read residual bytes.");
						Err(())
					},
				}
			},
		}
	}


	/// Read from unaligned memory
	pub fn read_mem8(&mut self, address: u32, n: usize) -> Result<Vec<u8>, ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, READMEM_8BIT };

		if n > self.max_packet {
			error!("Cannot read more than {} B at once.", self.max_packet);
			return Err(());
		}

		self.cmd_setup(n as u32, Direction::In);

		self.push_command(DEBUG_COMMAND);
		self.push_command(READMEM_8BIT);

		// Send address
		buf_write_u32(&mut self.cmdbuf, self.cmdidx + 0, address as u32, true);
		// Send transfer size
		buf_write_u16(&mut self.cmdbuf, self.cmdidx + 4, n as u16, true);
		self.cmdidx += 6;

		match self.recv(self.cmdidx, if n == 1 { n + 1 } else { n }, true) {
			Ok(_) => {
				let mut out = Vec::new();
				out.extend_from_slice(&self.databuf[0..n]);
				Ok( out )
			},

			_ => {
				error!("Could not read memory");
				Err(())
			}
		}
	}

	/// Read from aligned memory and a word aligned size
	pub fn read_mem32(&mut self, address: u32, n: usize) -> Result<Vec<u8>, ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, READMEM_32BIT };

		match (address % 4, n % 4) {
			(0, 0) => (),
			(0, _) => {
				error!("Read Memory 32 bit protocol. Number of bytes to read must be a multiple of 4 (word aligned)");
				//return Err(());
			},
			(_, 0) => {
				error!("Read Memory 32 bit protocol. Address must be word aligned.");
				//return Err(());
			},
			_ => {
				error!("Read Memory 32 bit protocol. Address and size are misaligned.");
				//return Err(());
			},
		}

		self.cmd_setup(n as u32, Direction::In);

		self.push_command(DEBUG_COMMAND);
		self.push_command(READMEM_32BIT);

		// Send address
		buf_write_u32(&mut self.cmdbuf, self.cmdidx + 0, address as u32, true);
		// Send transfer size
		buf_write_u16(&mut self.cmdbuf, self.cmdidx + 4, n as u16, true);
		self.cmdidx += 6;

		match self.recv(self.cmdidx, n, true) {
			Ok(_) => {
				let mut out = Vec::new();
				out.extend_from_slice(&self.databuf[0..n]);
				Ok( out )
			},

			_ => {
				error!("Could not read memory");
				Err(())
			}
		}
	}

	/// Unlock Flash by unlocking the flash register
	/// This is done by writing a sequence to the FLASH KEY Register
	pub fn unlock_flash(&mut self) -> Result<(), ()> {
		use super::super::constants::flash::{ register::{ CR, KEYR }, cr::{ LOCK } };
		use super::super::constants::flash::misc::{ KEY1, KEY2 };

		// First check that it's not already unlocked
		match self.read_debug_reg(CR) {
			Ok(cr) => match cr & LOCK {
				0 => Ok(()),
				_ => {
					debug!("Received FLASH CR: {:X}", cr);
					// Unlock flash register
					// Send KEY 1
					self.write_debug_reg(KEYR, KEY1)?;
					// Send KEY 2
					self.write_debug_reg(KEYR, KEY2)?;

					// Check that flash is unlocked
					match self.read_debug_reg(CR) {
						Ok(cr) => match cr & LOCK {
							0 => Ok(()),
							_ => {
								debug!("Received CR 0x{:X}", cr);
								error!("Flash could not be unlocked. Reason unknown.");
								Err(())
							},
						},
						_ => {
							error!("Reading Flash CR resulted in an error. Flash state is unknown.");
							Err(())
						},
					}
				},
			},
			_ => {
				error!("Could not unlock flash. Error when reading the flash register.");
				Err(())
			},
		}
	}
}



impl<'a> Link<'a> {
	/// Write memory in 32 bit mode
	pub fn write_mem32(&mut self, address: u32, data: &[u8]) -> Result<(), ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, WRITEMEM_32BIT };

		if data.len() > self.cmdbuf.len() {
			error!("Cannot send more data than the internal buffer.");
			return Err(());
		}

		match (address % 4, data.len() % 4) {
			(0, 0) => (),
			(0, _) => {
				error!("Read Memory 32 bit protocol. Number of bytes to read must be a multiple of 4 (word aligned)");
				return Err(());
			},
			(_, 0) => {
				error!("Read Memory 32 bit protocol. Address must be word aligned.");
				return Err(());
			},
			_ => {
				error!("Read Memory 32 bit protocol. Address and size are misaligned.");
				return Err(());
			},
		}

		self.cmd_setup(data.len() as u32, Direction::Out);

		self.push_command(DEBUG_COMMAND);
		self.push_command(WRITEMEM_32BIT);


		// Send address
		buf_write_u32(&mut self.cmdbuf, self.cmdidx + 0, address as u32, true);
		// Send transfer size
		buf_write_u16(&mut self.cmdbuf, self.cmdidx + 4, data.len() as u16, true);
		self.cmdidx += 6;

		for (i, b) in data.iter().enumerate() {
			self.databuf[i] = *b;
		}


		match self.send(self.cmdidx, data.len(), true) {
			Ok(_) => Ok(()),
			_ => {
				error!("Memory Write in 32 bit mode. Could not send data.");
				Err(())
			},
		}
	}

	/// Write memory in 8 bit mode
	/// Limited to 8 bytes per transfer
	pub fn write_mem8(&mut self, address: u32, data: &[u8]) -> Result<(), ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, WRITEMEM_8BIT };

		if data.len() > self.max_packet {
			error!("Cannot write more than {} Bytes in a transfer.", self.max_packet);
			return Err(());
		}

		self.cmd_setup(data.len() as u32, Direction::Out);
		self.push_command(DEBUG_COMMAND);
		self.push_command(WRITEMEM_8BIT);


		// Send address
		buf_write_u32(&mut self.cmdbuf, self.cmdidx + 0, address as u32, true);
		// Send transfer size
		buf_write_u16(&mut self.cmdbuf, self.cmdidx + 4, data.len() as u16, true);
		self.cmdidx += 6;

		match self.send(self.cmdidx, data.len(), true) {
			Ok(_) => Ok(()),
			_ => {
				error!("Memory Write in 32 bit mode. Could not send data.");
				Err(())
			},
		}
	}
}