//! Set and modify the speed of the connection

use crate::link::structs::SpeedMap;
use crate::link::enums::{ DebugMode, Cmd };
use crate::link::util::{ buf_read_u32 };
use super::Link;

use libusb::Direction;


/// SWD clock speed
pub const SWD_SPEED_MAP: [SpeedMap; 12] = [
	SpeedMap::new(4000, 0),
	// Default
	SpeedMap::new(1800, 1),
	SpeedMap::new(1200, 2),
	SpeedMap::new(950,  3),
	SpeedMap::new(480,  7),
	SpeedMap::new(240, 15),
	SpeedMap::new(125, 31),
	SpeedMap::new(100, 40),
	SpeedMap::new(50,  79),
	SpeedMap::new(25, 158),
	SpeedMap::new(15, 265),
	SpeedMap::new(5,  798),
];

pub const SWD_DEFAULT_SPEED: SpeedMap = SpeedMap::new(1800, 1);

/// JTAG clock speed
pub const JTAG_SPEED_MAP: [SpeedMap; 8] = [
	SpeedMap::new(18000, 2),
	SpeedMap::new(9000,  4),
	SpeedMap::new(4500,  8),
	SpeedMap::new(2250, 16),
	// Default
	SpeedMap::new(1125, 32),
	SpeedMap::new(562,  64),
	SpeedMap::new(281, 128),
	SpeedMap::new(140, 256),
];

pub const JTAG_DEFAULT_SPEED: SpeedMap = SpeedMap::new(1125, 32);

impl<'a> Link<'a> {
	/// Set the communication speed
	pub fn set_speed(&mut self, mode: DebugMode, khz: usize) -> Result<(), ()> {

		match mode {
			DebugMode::SWIM => self.speed_swim(khz),
			DebugMode::SWD => if self.version.jtag_api == 3 {
				self.speed_v3(false, khz)
			} else {
				self.speed_swd(khz)
			},
			DebugMode::JTAG => if self.version.jtag_api == 3 {
				self.speed_v3(true, khz) 
			} else {
				self.speed_jtag(khz)
			},
			_ => Err(()),
		}
	}

	/// Set the SWIM speed
	pub fn speed_swim(&mut self, khz: usize) -> Result<(), ()> {
		use super::super::constants::commands::swim::{ SWIM_COMMAND, SPEED };

		let speedcmd = match khz {
			0 => 0,
			_ => 1,
		};

		match self.command(0, Direction::In, vec![Cmd::Int8(SWIM_COMMAND), Cmd::Int8(SPEED), Cmd::Int8(speedcmd)]) {
			Ok(_) => Ok(()),
			_ => {
				error!("Set SWIM speed protocol. Could not set speed, bad command.");
				Err(())
			},
		}
	}

	/// Set the JTAG speed
	pub fn speed_jtag(&mut self, khz: usize) -> Result<(), ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, apiv2::JTAG_SET_FREQ };
		use super::super::constants::flags::HAS_JTAG_SET_FREQ;


		match self.version.flags & HAS_JTAG_SET_FREQ {
			0 => Ok(()),
			_ => match self.command(2, Direction::In, vec![Cmd::Int8(DEBUG_COMMAND), Cmd::Int8(JTAG_SET_FREQ), Cmd::Int16(closest_speed( khz, &JTAG_SPEED_MAP ).divisor as u16)]) {
				Ok(_) => {
					info!("Tried to match {} kHz, set speed: {} kHz", khz, closest_speed(khz, &JTAG_SPEED_MAP).speed);
					Ok(())
				},
				_ => {
					error!("Set JTAG speed protocol. Could not set speed. Bad command.");
					Err(())
				},
			},
		}
	}

	/// Set thw SWD speed
	pub fn speed_swd(&mut self, khz: usize) -> Result<(), ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, apiv2::SWD_SET_FREQ };
		use super::super::constants::flags::HAS_SWD_SET_FREQ;

		match self.version.flags & HAS_SWD_SET_FREQ {
			0 => Ok(()),
			_ => match self.command(2, Direction::In, vec![Cmd::Int8(DEBUG_COMMAND), Cmd::Int8(SWD_SET_FREQ), Cmd::Int16( closest_speed(khz, &SWD_SPEED_MAP).divisor as u16 )]) {
				Ok(_) => Ok(()),
				_ => {
					error!("Set SWD speed protocol. Could not set speed. Bad command.");
					Err(())
				},
			},
		}
	}

	/// Set speed for STLink V3 devices
	pub fn speed_v3(&mut self,is_jtag: bool, khz: usize) -> Result<(), ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, apiv3::SET_COM_FREQ };

		let map = self.get_com_freq(is_jtag)?;
		let jtagcmd = if is_jtag { 1 } else { 0 };

		match self.command(8, Direction::In,
			vec![Cmd::Int8(DEBUG_COMMAND),
				Cmd::Int8(SET_COM_FREQ),
				Cmd::Int8(jtagcmd),
				Cmd::Int8(0),
				Cmd::Int32( closest_speed(khz, &map).speed as u32 )
			])
		{
			Ok(_) => Ok(()),
			_ => {
				error!("STLink V3 set speed protocol. Bad command.");
				Err(())
			},
		}


	}


	/// Method for getting com freq
	pub fn get_com_freq(&mut self, is_jtag: bool) -> Result<Vec<SpeedMap>, ()> {
		use super::super::constants::commands::debug::{ DEBUG_COMMAND, apiv3::GET_COM_FREQ };

		match self.version.jtag_api {
			3 => {
				let jtagcmd = if is_jtag { 1 } else { 0 };

				match self.command(52, Direction::In, vec![Cmd::Int8(DEBUG_COMMAND), Cmd::Int8(GET_COM_FREQ), Cmd::Int8(jtagcmd)]) {
					Ok(_) => {
						let size = if self.databuf[8] > super::super::constants::misc::V3::MAX_FREQ_NB {
							super::super::constants::misc::V3::MAX_FREQ_NB
						} else {
							self.databuf[8]
						};

						let mut map: Vec<SpeedMap> =
							(0usize..size as usize)
								.map(|i| SpeedMap { speed: buf_read_u32(&self.databuf, 12 + 4*i, true) as usize, divisor: i } )
								.collect();
						map.extend( (size..super::super::constants::misc::V3::MAX_FREQ_NB).map(|i| SpeedMap { speed: 0, divisor: i as usize } ) );


						Ok(map)
					},
					_ => {
						error!("STLink V3 Get Comm Frequency protocol. Failed to receive data.");
						Err(())
					},
				}

			},
			_ => Err(()),
		}
	}
}


fn closest_speed(khz: usize, map: &[SpeedMap]) -> SpeedMap {
	if map.len() == 0 {
		error!("Cannot map through 0 sized array");
	}
	let mut delta = !0;
	for speed in map {
		let new = if khz > speed.speed {
			khz - speed.speed
		} else {
			speed.speed - khz
		};

		if new < delta {
			delta = new;
			continue;
		} else {
			return speed.clone();
		}
	}

	SpeedMap { speed: 0, divisor: 1 }
}