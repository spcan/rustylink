//! Flash interaction

use crate::link::util::{ buf_write_u32, buf_read_u32, buf_write_u16 };

use super::super::enums::Cmd;

use libusb::Direction;

use super::Link;

impl<'a> Link<'a> {
	/// Read `n` bytes from an `address` in memory
	/// The reads are done preferably in 32 bit accesses in 4 kB bursts
	/// until it is not possible to do the 32 bit read (due to misalignment).
	/// Then it defaults to using 8 bit access. 16 bit access are not supported
	/// because almost no devices suport them. In the future they may get a dedicated 
	/// method.
	pub fn read_mem2(&mut self, address: u32, n: usize) -> Result<Vec<u8>, ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, READMEM_32BIT, READMEM_8BIT };

		// TODO : Check that the current board supports 32 bit access, STM8 may not support it??

		// Check if data is multiple of 4 and word (32 bit) aligned
		// By the implementation, the data must be aligned for general method, specific methods may
		// be left unaligned.
		let is_word_aligned = match (n % 4, address % 4) {
			(0, 0) => true,
			(_, 0) => {
				warn!("Reading non-32 bit memory");
				false
			},
			_ => {
				error!("Cannot read 32 bit memory if memory is not 32 bit aligned");
				return Err(());
			},
		};

		let runs = n / self.databuf.len();
		let res = n % self.databuf.len();

		let mut out = Vec::new();

		debug!("Data transfer planned in {} bursts of {} kB and one burst of {} B", runs, self.databuf.len() / 1024, res);

		// Do the full data buffer transfers
		// These transfers are done in 32 bit mode because the data buffer size is 32 bit aligned
		for i in 0..runs {
			// Set up commands
			self.cmd_setup(self.databuf.len() as u32, Direction::In);
			self.push_command(DEBUG_COMMAND);

			// Send command. Read Memory 32 bit
			self.push_command(READMEM_32BIT);

			// Send address
			// The data buffer length is always 32 bit aligned
			buf_write_u32(&mut self.cmdbuf, self.cmdidx + 0, address + (i * self.databuf.len()) as u32, true);
			// Send transfer size
			buf_write_u16(&mut self.cmdbuf, self.cmdidx + 4, self.databuf.len() as u16, true);
			self.cmdidx += 6;

			// Send command -> Receive data
			match self.recv(self.cmdidx, self.databuf.len(), true) {
				Ok(_) => out.extend_from_slice(&self.databuf),
				_ => {
					error!("Error during flash memory reading. Failed in burst transfer {} with a transfer size of {}.", i, self.databuf.len());
					return Err(());
				},
			}
		}

		// Do the residual transfer
		if res != 0 {
			// Calculate the part that can be done in 32 bit mode.
			let (size32, size8) = (res - (res%4), res % 4);

			// Set up command
			self.cmd_setup(size32 as u32, Direction::In);
			self.push_command(DEBUG_COMMAND);

			// Send command. Read memory 32 bit
			self.push_command(READMEM_32BIT);

			// Send address
			// The data buffer length is always 32 bit aligned
			buf_write_u32(&mut self.cmdbuf, self.cmdidx + 0, address + (runs * self.databuf.len()) as u32, true);
			// Send transfer size
			buf_write_u16(&mut self.cmdbuf, self.cmdidx + 4, size32 as u16, true);
			self.cmdidx += 6;

			// Send command -> Receive data
			match self.recv(self.cmdidx, size32, true) {
				Ok(_) => {
					debug!("Aligned transfer complete!");
					out.extend_from_slice(&self.databuf[0..size32]);
				},
				_ => {
					error!("Error during flash memory reading. Failed in residual transfer of size {} when reading in 32 bit mode.", size32);
					return Err(());
				},
			}

			// If it's not word aligned, read 1, 2 or 3 single bytes
			if !is_word_aligned {
				// Set up command
				self.cmd_setup(size8 as u32, Direction::In);
				self.push_command(DEBUG_COMMAND);

				// Send command. Read memory 8 bit
				self.push_command(READMEM_8BIT);

				// Send address
				buf_write_u32(&mut self.cmdbuf, self.cmdidx + 0, address + (runs * self.databuf.len() + size32) as u32, true);
				// Send transfer size
				buf_write_u16(&mut self.cmdbuf, self.cmdidx + 4, size32 as u16, true);
				self.cmdidx += 6;

				// Send command -> Receive data
				match self.recv(self.cmdidx, size8, true) {
					Ok(_) => {
						debug!("Transfer complete");
						out.extend_from_slice(&self.databuf[0..size8]);
					},
					_ => {
						error!("Error during flash memory reading. Failed in residual transfer when reading {} residual bytes.", size8);
						return Err(());
					}
				}
			}
		}

		Ok( out )
	}

	/// Writes from a given `buffer` (internal or external) in 32 bit mode
	/// into the internal device flash at the given `address`
	pub fn write_mem(&mut self, address: u32, buffer: &[u8]) -> Result<(), ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, WRITEMEM_32BIT };

		// Data must be a multiple of 4 and word (32 bit) aligned
		match (buffer.len() % 4, address % 4) {
			(0, 0) => (),
			(0, _) => {
				error!("Cannot read 32 bit memory if memory is not 32 bit aligned");
				return Err(());
			},
			(_, 0) => {
				error!("Cannot read incomplete 32bit memory (n % 4 != 0)");
				return Err(());
			},
			_ => {
				error!("Memory is not aligned and not in 32 bit sections");
				return Err(());
			},
		}

		// Transfers are done in 4 kB bursts
		let runs = buffer.len() / 4096;
		let res = buffer.len() % 4096;

		debug!("Data transfer planned in {} bursts of 4 kB and one burst of {} B", runs, res);

		// Do the 4096 Bytes transfers
		for i in 0..runs {
			// Set up commands
			self.cmd_setup(4096, Direction::In);
			self.push_command(DEBUG_COMMAND);

			// Send command. Write to memory in 32 bit
			self.push_command(WRITEMEM_32BIT);

			// Send address
			buf_write_u32(&mut self.cmdbuf, self.cmdidx + 0, address + (i * 4096) as u32, true);
			// Send transfer size
			buf_write_u16(&mut self.cmdbuf, self.cmdidx + 4, 4096, true);
			self.cmdidx += 6;

			// Send command -> Send data
			match self.send(self.cmdidx, self.databuf.len(), true) {
				Ok(_) => (),
				_ => {
					error!("Error during flash memory reading.");
					return Err(());
				},
			}
		}

		// Do the residual transfer
		if res != 0 {
			// Set up commands
			self.cmd_setup(res as u32, Direction::In);
			self.push_command(DEBUG_COMMAND);

			// Send command. Write to memory in 32 bit
			self.push_command(WRITEMEM_32BIT);

			// Send address
			buf_write_u32(&mut self.cmdbuf, self.cmdidx + 0, address + (runs * 4096) as u32, true);
			// Send transfer size
			buf_write_u16(&mut self.cmdbuf, self.cmdidx + 4, res as u16, true);
			self.cmdidx += 6;

			// Send command -> Send data
			match self.recv(self.cmdidx, self.databuf.len(), true) {
				Ok(_) => {
					debug!("Transfer complete!");
					Ok(())
				},
				_ => {
					error!("Error during flash memory reading.");
					Err(())
				},
			}
		} else {
			// If there is no residual transfer return
			Ok(())
		}
	}

	/// Read memory in 8 bit (1 byte) increments
	pub fn read_mem8(&mut self, address: u32, len: usize) -> Result<Vec<u8>, ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, READMEM_8BIT };

		if len > self.max_packet {
			error!("Cannot read in quantities over the max allowed size!");
			return Err(());
		}

		match self.command(if len == 1 { len + 1 } else { len }, Direction::In,
			vec![Cmd::Int8(DEBUG_COMMAND),
				Cmd::Int8(READMEM_8BIT),
				Cmd::Int32(address),
				Cmd::Int16(len as u16)
			])
		{
			Ok(_) => match self.rw_status() {
				Ok(_) => {
					let mut out = Vec::new();
					out.extend_from_slice(&self.databuf[0..len]);
					Ok( out )
				},
				_ => {
					error!("RW status reading failed.");
					Err(())
				},
			},
			_ => {
				error!("Could not read memory. Error during transmission.");
				Err(())
			},
		}
	}

	/// Read memory in 32 bit increments
	pub fn read_mem32(&mut self, address: u32, len: usize) -> Result<Vec<u8>, ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, READMEM_32BIT };

		// Data must be a multiple of 4 and word aligned
		match (len % 4, address % 4) {
			(0, 0) => {
				match self.command(len, Direction::In,
					vec![Cmd::Int8(DEBUG_COMMAND),
						Cmd::Int8(READMEM_32BIT),
						Cmd::Int32(address),
						Cmd::Int16(len as u16)
					])
				{
					Ok(_) => match self.rw_status() {
						Ok(_) => {
							let mut out = Vec::new();
							out.extend_from_slice(&self.databuf[0..len]);
							Ok( out )
						},
						_ => {
							error!("RW status reading failed.");
							Err(())
						},
					},
					_ => {
						error!("Could not read memory. Error during transmission.");
						Err(())
					},
				}
			},
			(0, _) => {
				error!("Data read is not word aligned!");
				Err(())
			},
			(_, 0) => {
				error!("Requesting a partial 32bit integer!");
				Err(())
			},
			_ => {
				error!("Data is not aligned & requesting a partial 32 bit integer!");
				Err(())
			},
		}

	}


	pub fn write_mem32(&mut self, address: u32, value: u32) -> Result<(), ()> {
		let mut temp = [0; 4];
		buf_write_u32(&mut temp, 0, value, true);

		self.write_mem(address, &temp)
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

	/// Check the status of the last memory transfer
	pub fn rw_status(&mut self) -> Result<usize, ()> {
		match self.version.jtag_api {
			1 => Ok(0),
			_ => {
				self.cmd_setup(2, Direction::In);
				self.push_command(super::super::constants::commands::debug::DEBUG_COMMAND);

				let (len, cmd) = match self.version.flags & super::super::constants::flags::HAS_GETLASTRWSTATUS2 {
					0 => ( 2, super::super::constants::commands::debug::apiv2::GETLASTRWSTATUS ),
					_ => (12, super::super::constants::commands::debug::apiv2::GETLASTRWSTATUS2),
				};

				self.recv(self.cmdidx, len, true)
			},
		}
	}
}

impl<'a> Link<'a> {
	/// Read memory from the device
	/// `address` : address from which to start reading
	/// `n` : number of bytes to read
	pub fn read_mem(&mut self, address: u32, n: usize) -> Result<Vec<u8>, ()> {
		use super::super::constants::flags::HAS_MEM_16BIT;

		if n > self.databuf.len() {
			error!("Cannot read more than internal buffer.");
			return Err(());
		}

		let mut out = Vec::new();

		// Prioritize 32 bit readings, as they are not limited
		// 1. Align the memory address. Read bytes until aligned.
		// 2. Read in 32 bit mode until it can no longer read words.
		// 3. Finish reading the last bytes.
		let realign = address % 4;

		let (newaddress, nbytes) = match realign as usize {
			0 => (address, n),
			r => {
				match self.read_mem8(address, 4 - r) {
					Ok(_) => out.extend_from_slice(&self.databuf[0..4-r]),
					_ => {
						error!("Error while reading unaligned memory. Could not read first bytes.");
						return Err(());
					},
				}

				(address + (4 - realign), n - (4 - r) )
			},
		};

		match newaddress % 4 {
			0 => (),
			_ => {
				error!("Address was not realigned.");
				return Err(());
			},
		}

		match self.read_mem32(newaddress, nbytes - (nbytes % 4)) {
			Ok(_) => out.extend_from_slice(&self.databuf[0..(nbytes - (nbytes % 4))]),
			_ => {
				error!("Error while doing a mass transfer in 32 bit mode.");
				return Err(());
			},
		}

		match nbytes % 4 {
			0 => Ok( out ),
			b => match self.read_mem8(newaddress + nbytes as u32 - b as u32, b) {
				Ok(_) => {
					out.extend_from_slice(&self.databuf[0..b]);
					Ok( out )
				},
				_ => {
					error!("Error while doing residual transfer.");
					Err(())
				},
			},
		}
	}
}