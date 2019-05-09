//! Link methods that give information about the 
//! device and its state

use crate::link::structs::{ FlashInfo, SysMemInfo };
use crate::link::enums::{ STLinkMode, DebugMode, Cmd };
use crate::link::util::{ buf_read_u32, buf_read_u16 };

use libusb::Direction;

use super::Link;

impl<'a> Link<'a> {
	/// Get the target's voltage level
	pub fn voltage(&mut self) -> Result<f32, ()> {
		use super::super::constants::{ flags::HAS_TARGET_VOLT, commands::GET_TARGET_VOLTAGE };

		match self.version.flags & HAS_TARGET_VOLT {
			0 => return Ok(0.0),
			_ => (),
		}

		match self.command(8, Direction::In, vec![Cmd::Int8(GET_TARGET_VOLTAGE)]) {
			Ok(n) => match n {
				8 => {
					let div = buf_read_u32(&self.databuf, 0, true);
					let mul = buf_read_u32(&self.databuf, 4, true);

					Ok( ( 2.4 * (mul as f32) )  / (div as f32) )

				},
				b => {
					error!("Voltage reading protocol. Expected 8 bytes, received {}", b);
					Err(())
				},
			},
			Err(_) => {
				error!("Voltage reading protocol. Data request failed.");
				Err(())
			},
		}
	}


	/// Get the USB current mode
	pub fn current_mode(&mut self) -> Result<STLinkMode, ()> {
		match self.command(2, Direction::In, vec![Cmd::Int8(0xF5)]) {
			Ok(_) => {
				//debug!("Get USB Mode protocol. Received {} bytes.", n);
				//debug!("First four u32: {:?}", &self.databuf[0..16]);
				Ok(
					match self.databuf[0] {
						0x0 => STLinkMode::DFU,
						0x1 => STLinkMode::Mass,
						0x2 => STLinkMode::Debug(DebugMode::Unknown),
						0x3 => STLinkMode::Debug(DebugMode::SWIM),
						_ => STLinkMode::Unknown,
					}
				)
			},
			Err(_) => {
				error!("Get USB Mode protocol. Reading failed.");
				Err(())
			}
		}
	}

	/// Get the core ID
	pub fn core_id(&mut self) -> Result<u32, ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND };

		let (offset, readcore, rxsize) = match self.version.jtag_api {
			1 => (0, super::super::constants::commands::debug::READCOREID, 4),
			_ => (4, super::super::constants::commands::debug::apiv2::READ_IDCODES, 12),
		};

		match self.command(rxsize, Direction::In, vec![Cmd::Int8(DEBUG_COMMAND), Cmd::Int8(readcore)]) {
			Ok(_) => Ok(buf_read_u32(&self.databuf, offset, true)),
			Err(_) => {
				error!("Get Core ID protocol. Reading failed.");
				Err(())
			},
		}
	}

	/// Get the device status
	pub fn status(&mut self) -> Result<u32, ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, GETSTATUS };

		match self.command(2, Direction::In, vec![Cmd::Int8(DEBUG_COMMAND), Cmd::Int8(GETSTATUS)]) {
			Ok(_) => Ok(self.databuf[0] as u32),
			_ => {
				error!("Get status protocol. Reading failed.");
				Err(())
			},
		}
	}

	/// Get the Chip info
	/// It gets all info for the link to be able to map memory correctly
	/// Returns the chip ID if successful
	pub fn get_chip_info(&mut self) -> Result<u32, ()> {
		match self.read_debug_reg(0xE004_2000) {
			Ok(chipid) => {
				//self.chipid = super::super::chipid::get_chip_from_id_u32(chipid),
				let chip = super::super::chipid::get_chip_from_id_u32(chipid & mask!(11));
				match chip.id {
					0 => Err(()),
					_ => {
						let size = buf_read_u16( &self.read_mem8(chip.flash_size_reg, 2)?, 0, true );
						self.memory.flash = FlashInfo { base: 0x0800_0000, size: size as u32, pagesize: if size as u32 == chip.pagesize { None } else { Some(chip.pagesize) } };
						self.memory.ram = chip.sram;
						self.memory.sys = SysMemInfo { base: chip.bootrom_base, size: chip.bootrom_size };

						info!("Recognized chip as a: {:?}", chip.description);
						info!("It has {} kB of Flash at address 0x{:X}", size, 0x0800_0000);
						info!("It has {} SRam sections", self.memory.ram.len());
						for (i, r) in self.memory.ram.iter().enumerate() {
							info!("  Section {}: {} kB at address 0x{:X}", i + 1, r.size as f32 / 1024.0, r.base);
						}
						info!("Boot/system memory has a size of {} kB at address 0x{:X} ", self.memory.sys.size as f32 / 1024.0, self.memory.sys.base);

						Ok(chipid & mask!(11))
					},
				}
			},
			_ => {
				error!("Failure to get chip id");
				Err(())
			},
		}
	}
}