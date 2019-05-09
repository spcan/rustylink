//! STLink module


mod sendrecv;
mod registers;
mod flash;
mod debug;
mod info;
mod reset;
mod speed;
mod modes;


use crate::link::link::speed::{ JTAG_DEFAULT_SPEED, SWD_DEFAULT_SPEED };
use libusb::Direction;

use super::structs::{ STLinkUSBVersion, Endpoint, MemInfo };

use libusb::{ DeviceHandle };


use super::constants::{ misc::{ SIZE::{ DATA, SG } } };

use super::enums::{ STLinkMode, DebugMode };


use super::util::{ buf_write_u32, buf_read_u32, buf_read_u16, buf_write_u16 };


pub struct Link<'a> {
	tx: u8,
	rx: u8,
	trace: u8,

	handle: DeviceHandle<'a>,

	cmdbuf: [u8; SG],
	cmdidx: usize,

	databuf: [u8; DATA],

	version: STLinkUSBVersion,

	vid: u16,
	pid: u16,

	sg_transfer: usize,

	max_packet: usize,

	memory: MemInfo,
}

impl<'a> Link<'a> {
	pub fn dump_data(&self, offset: usize) {
		print!("[");
		for b in &self.databuf[0..offset] {
			print!("{}, ", b);
		}
		println!("]");
	}

	pub fn enter_debug_state(&mut self) -> Result<(), ()> {
		use super::constants::commands::debug::DEBUG_COMMAND;

		self.cmd_setup(16, Direction::In);
		self.push_command(DEBUG_COMMAND);
		self.push_command(0x35);
		self.push_command(0xF0);
		self.push_command(0xED);
		self.push_command(0x00);
		self.push_command(0xE0);
		self.push_command(0x03);
		self.push_command(0x00);
		self.push_command(0x5F);
		self.push_command(0xA0);

		match self.recv(self.cmdidx, 100, true) {
			Ok(_) => Ok(()),
			_ => Err(())
		}
	}
}


/// Debug methods
impl<'a> Link<'a> {

	/// Set SWD Clock
	pub fn set_swdclk(&mut self, divisor: u16) -> Result<(), ()> {
		if self.version.stlink >= 2 && self.version.jtag_api == 3 {
			self.cmd_setup(2, Direction::In);
			self.push_command(super::constants::commands::debug::DEBUG_COMMAND);
			self.push_command(super::constants::commands::debug::apiv2::SWD_SET_FREQ);
			self.push_command((divisor >> 0) as u8);
			self.push_command((divisor >> 8) as u8);

			match self.recv(self.cmdidx, 2, true) {
				Ok(_) => {
					Ok(())
				},
				Err(_) => {
					error!("Set SWD Clock protocol. Writing failed.");
					Err(())
				},
			}
		} else {
			Err(())
		}
	}
}

/// Methods to connect through USB
impl<'a> Link<'a> {
	/// Open USB connection
	pub fn open_usb(ctx: &'a mut libusb::Context, retries: usize, connection: DebugMode) -> Result<Self, ()> {
		let mut tries = retries;
		let mut new = loop {
			// Search for a correct interface
			let (device, desc, mut handle) = match crate::usb::search(ctx) {
				Some(a) => a,
				None => {
					error!("No results found for STLink devices");
					return Err(());
				},
			};


			let cfg = match device.config_descriptor(0) {
				Ok(c) => c,
				_ => panic!(),
			};

			let mut eps = Vec::new();

			// Loop trough the interfaces
			// Rewrite with maps??
			for interface in cfg.interfaces() {
				for idesc in interface.descriptors() {
					for endpoint in idesc.endpoint_descriptors() {
						eps.push(Endpoint {
							config: cfg.number(),
							iface: idesc.interface_number(),
							setting: idesc.setting_number(),
							address: endpoint.address(),
						});
					}
				}
			}

			match eps.len() {
				3 => (),
				l => {
					error!("Device received does not have the correct number of endpoints. Expected 3 found {}", l);
					return Err(());
				},
			}

			/*for endpoint in eps.iter() {
				(handle.claim_interface(endpoint.iface));
				(handle.set_alternate_setting(endpoint.iface, endpoint.setting));
			}*/

			handle.claim_interface(eps[0].iface);
			handle.set_alternate_setting(eps[0].iface, eps[0].setting);

			info!("libusb found device VID:PID {:X}:{:X}", desc.vendor_id(), desc.product_id());

			let (tx, stlink, trace) = match desc.product_id() {
				// STLink V1
				0x3744 => (2 | 0x00, 1, None),
				// STLink V3
				0x374d | 0x374e | 0x374f | 0x3753 => (1 | 0x00, 3, Some( 2 | 0x80 ) ),
				// STLink V2.1
				0x374b | 0x3752 => (1 | 0x00, 2, Some( 2 | 0x80 ) ),
				// STLink V2
				0x3748 => (2 | 0x00, 2, Some( 3 | 0x80 ) ),
				// Default will be STLink V2
				_ => (2 | 0x00, 2, Some( 3 | 0x80 ) ),
			};


			let mut new = Self {
				tx: tx,
				// `rx` is always the same for all versions
				rx: 0x80 | 1,
				trace: match trace { Some(t) => t, None => 0, },

				handle: handle,
				
				// Command buffer, may use STLinkV2 Size or STLinkV1 size
				// Unique size buffer, the actual command size is stored in cmdidx.
				cmdbuf: [0; 31],
				// Index of the next command to push. Is equal to the size of the command
				cmdidx: 0,
				// A 4kB buffer for data transmission.
				databuf: [0; 4096],

				version: STLinkUSBVersion { stlink:stlink, jtag: 0, swim: 0, jtag_api: 1, flags:0 },

				vid: desc.vendor_id(),
				pid: desc.product_id(),

				sg_transfer: 0,

				// Default size
				max_packet: 64,

				memory: MemInfo::new(),
			};

			match new.version() {
				Ok(_) => break new,
				_ => match new.version.stlink {
					1 if tries == 0 => {
						error!("Reached max number of retries.\n");
						return Err(());
					},
					_ => {
						//new.emergency_reset();
						if tries == 0 {
							return Err(());
						} else {
							tries -= 1;
						}
						std::thread::sleep(std::time::Duration::from_secs(1));
						continue;
					},
				},
			}
		};

		info!("Setting up connection in {:?} mode", connection);

		match connection {
			DebugMode::SWD  => if new.version.jtag_api == 1 { return Err(()); } else { () },
			DebugMode::JTAG => if new.version.jtag_api == 0 { return Err(()); } else { () },
			DebugMode::SWIM => if new.version.swim     == 0 { return Err(()); } else { () },
			_ => {
				error!("USB connection protocol. Debug Mode is not supported: {:?}.", connection);
				return Err(());
			},
		}

		// TODO : Change this to prepare for resets
		info!("Initializing...");
		match new.init_mode(STLinkMode::Debug(connection), false) {
			Ok(_) => (),
			_ => {
				error!("Could not set up STLink connection.");
				return Err(());
			},
		}

		// Unlock flash
		new.unlock_flash();
		// Get the chip info to perform correct memory operations
		new.get_chip_info();

		// Get the max packet size available
		// If it's SWIM mode, the size is predetermined
		if connection == DebugMode::SWIM {
			new.enter_mode(DebugMode::SWIM)?;
			new.max_packet = 4096;
		} else {
			new.max_packet = 1 << 10;

			match new.read_mem32(super::constants::address::STM32::CPUID, 12) {
				Ok(_) => {
					new.dump_data(12);
					let cpuid = buf_read_u32(&new.databuf, 0, true);
					match (cpuid >> 4) & 0xF {
						4 | 3 => new.max_packet = 1 << 12,
						_ => (),
					}

					debug!("CPUID: 0x{:X}", cpuid);
					debug!("       {}", cpuid);
				},
				Err(_) => {
					error!("Could not read max packet size.");
					return Err(());
				},
			}
		}

		info!("Max packet size: {} kB", new.max_packet as f32 / 1024.0);

		match new.status() {
			Ok(s) => info!("Device status: {}", s),
			_ => (),
		}

		// Do not halt yet, Core ID has not been requested
		//new.halt();

		Ok(new)
	}

	/// Check version through USB and update itself if there's not an error
	pub fn version(&mut self) -> Result<(), ()> {
		// Set up `GET_VERSION` command
		self.cmd_setup(6, Direction::In);
		self.push_command( super::constants::commands::GET_VERSION );
		
		match self.recv(self.cmdidx, 6, true) {
			Ok(_) => (),
			_ => {
				error!("USB Version protocol. Error during version data request");
				return Err(());
			},
		}

		// Extract data
		let version = buf_read_u16(&self.databuf, 0, false);
		let mut vid = buf_read_u16(&self.databuf, 2, true);
		let mut pid = buf_read_u16(&self.databuf, 4, true);

		let mut v = (version >> 12) & 0x0f;
		let x = (version >>  6) & 0x3f;
		let y = (version >>  0) & 0x3f;

		let mut bridge = 0;


		let (mut jtag, mut swim, mut msd) = match pid {
			// STLink V2.1 | STLink V2.1 No MSD
			0x3752 | 0x374B => match (x, y) {
				(0...22, 7) | (25...255, 7...12) => (0, y, x),
				_ => (x, 0, y),
			},
			_ => (x, y, 0),
		};

		match (v, x, y) {
			(3, 0, 0) => {
				self.cmd_setup(16, Direction::In);
				// STLink V3 Get USB Extended command
				self.push_command(0xFB);

				match self.recv(self.cmdidx, 12, true) {
					Ok(_) => {
						v = self.databuf[0] as u16;
						swim = self.databuf[1] as u16;
						jtag = self.databuf[2] as u16;
						msd  = self.databuf[3] as u16;
						bridge = self.databuf[4];

						vid = buf_read_u16(&self.databuf,  8, true);
						pid = buf_read_u16(&self.databuf, 10, true);
					},
					Err(_) => {
						error!("USB Version protocol. Failed while getting STLink V3 extended info\n");
						return Err(());
					},
				}

			},
			_ => (),
		}

		// Store version data
		self.version.stlink =    v as usize;
		self.version.jtag   = jtag as usize;
		self.version.swim   = swim as usize;

		// Set up flags
		self.version.flags = match self.version.stlink {
			1 => match self.version.jtag {
				0...10 => {
					self.version.jtag_api = 1;
					0
				},
				_ => {
					self.version.jtag_api = 2;
					0
				},
			},
			2 => {
				self.version.jtag_api = 2;
				0 |
				if self.version.jtag >= 13 { super::constants::flags::HAS_TRACE            } else { 0 } |
				if self.version.jtag >= 15 { super::constants::flags::HAS_GETLASTRWSTATUS2 } else { 0 } |
				if self.version.jtag >= 22 { super::constants::flags::HAS_SWD_SET_FREQ     } else { 0 } |
				if self.version.jtag >= 24 { super::constants::flags::HAS_JTAG_SET_FREQ    } else { 0 } |
				if self.version.jtag >= 26 { super::constants::flags::HAS_MEM_16BIT        } else { 0 }  
			},

			3 => {
				self.version.jtag_api = 3;
				super::constants::flags::HAS_TRACE            |
				super::constants::flags::HAS_GETLASTRWSTATUS2 |
				super::constants::flags::HAS_MEM_16BIT
			},
			_ => 0,
		};

		info!("Using version STLink V{} J{} M{} B{} S{} (API {}) VID:PID {:#04X}:{:#04X}", v, jtag, msd, bridge, swim, self.version.jtag_api, vid, pid);

		if self.vid != vid || self.pid != pid {
			error!("USB Version protocol. Discrepancy in VID:PID, expected {:#04X}:{:#04X} found {:#04X}:{:#04X}", self.vid, self.pid, vid, pid);
		}

		Ok(())
	}


	/// Connect to device using the given `mode`
	/// TODO : `connect_under_reset` is not working yet, maybe try to enable it later on in development
	pub fn init_mode(&mut self, mode: STLinkMode, connect_under_reset: bool) -> Result<(), ()> {
		// Get current mode
		// If it's unknown it may indicate a software error coming from before this method was called
		// If it's DFU mode (USB standard), exit this mode
		match self.current_mode() {
			Ok(m) => match m {
				STLinkMode::Unknown => warn!("Initial Mode protocol. Unknwon initial mode."),
				STLinkMode::DFU => {
					match self.leave_mode(STLinkMode::DFU) {
						Ok(_) => (),
						Err(_) => {
							error!("Initial Mode protocol. Could not exit DFU mode.");
							return Err(());
						},
					}
				},
				_ => (),
			},
			Err(_) => {
				error!("Initial Mode protocol. Could not get current mode.");
				return Err(());
			},
		}

		// Check mode again because we may have changed to a default one
		// If it's still DFU, something's wrong. DFU mode can be supported (refer to STM datasheets and manuals)
		// but is not supported in this version.
		// If it's any other mode, check the voltage in the device to see if debugging can happen
		match self.current_mode() {
			Ok(m) => match m {
				STLinkMode::DFU => {
					error!("DFU Mode support is not supported!");
					return Err(());
				},
				_ => match self.voltage() {
					Ok(v) => match v {
						x if x < 1.5 => warn!("Voltage is too low to perform accurate debugging!"),
						_ => info!("Target voltage level: {} V", v),
					},

					_ => {
						error!("Initial Mode protocol. Could not get target voltage.");
						return Err(());
					},
				}
			},
			Err(_) => {
				error!("Initial Mode protocol. After clearing mode, it could not get current mode.");
				return Err(());
			},
		}

		// Get the given Debug mode
		let dbgmode = match mode {
			STLinkMode::Debug(m) => match m {
				DebugMode::Unknown => {
					error!("Initial Mode protocol. Could not set mode {:?}", mode);
					return Err(());
				},
				dbg => dbg,
			},
			n => {
				error!("Initial mode protocol. Error while setting mode. Mode is not permitted: {:?}", n);
				return Err(());
			},
		};

		// Set interface speed if possible
		match dbgmode {
			DebugMode::JTAG => match self.version.jtag {
				0 => {
					error!("Initial Mode protocol. Device does not support JTAG.");
					return Err(());
				},
				_ => match self.version.flags & super::constants::flags::HAS_JTAG_SET_FREQ {
					0 => info!("Device cannot set speed for JTAG interface."),
					_ => match self.set_speed(dbgmode, JTAG_DEFAULT_SPEED.divisor) {
						Ok(_) => (),
						_ => error!("Initial Mode protocol. Could not set JTAG interface speed. Continuing."),
					},
				},
			},
			DebugMode::SWD => match self.version.flags & super::constants::flags::HAS_SWD_SET_FREQ {
				0 => info!("Device cannot set speed for SWD interface."),
				_ => match self.set_speed(dbgmode, SWD_DEFAULT_SPEED.divisor) {
					Ok(_) => (),
					_ => error!("Initial Mode protocol. Could not set SWD interface speed. Continuing."),
				}
			}
			d => {
				error!("Illegal mode {:?}. This mode is not supported as an interface.", d);
				return Err(());
			},
		}

		match self.version.jtag_api {
			3 => match self.set_speed(dbgmode, match dbgmode {
				DebugMode::JTAG => JTAG_DEFAULT_SPEED.speed,
				DebugMode::SWD => SWD_DEFAULT_SPEED.speed,
				_ => unreachable!(),
			})
			{
				Ok(_) => (),
				_ => {
					error!("Initial Mode protocol. Could not set STLink V3 speed.");
					return Err(());
				},
			},
			_ => (),
		}

		// Preliminary SRST assert:
		// We want SRST is asserted before activating debug signals (mode_enter).
		if connect_under_reset && (mode != STLinkMode::Debug(DebugMode::SWIM)) {
			self.assert_srst(mode, 0)?;
		}

		// Enter the given Debug mode. Only modes accepted are JTAG and SWD
		match self.enter_mode(dbgmode) {
			Ok(_) => (),
			_ => {
				error!("Initial Mode protocol. Could not enter {:?} mode.", dbgmode);
				return Err(());
			},
		}



		// Assert reset again, making sure this time the correct pin is used
		// Before, the result may be botched
		if connect_under_reset {
			self.assert_srst(mode, 0)?;
		}


		// Assert there is a Debug mode
		match self.current_mode() {
			Ok(m) => match m {
				STLinkMode::Debug(d) if d == dbgmode => info!("Device is now in {:?} mode", d),
				d => error!("Tried to set device in {:?} mode, device is in {:?} mode", dbgmode, d),
			},
			_ => error!("Error while checking device mode. Cannot make sure if set up is correct."),
		}

		Ok(())
	}
}


/// Helper functions
impl<'a> Link<'a> {
	/// Terminate transmission
	pub fn terminate(&mut self) -> Result<(), ()> {
		use super::constants::misc::TIMEOUT::WRITE as WriteTimeout;
		match self.version.stlink {
			1 => {
				// SG buffer, 13 bytes
				let mut buf = Vec::with_capacity(13);
				match self.handle.read_bulk(self.rx, &mut buf, WriteTimeout) {
					Ok(n) => {
						if n != 13 {
							debug!("Terminate protocol. SG buffer not filled. Expected 13 bytes, received {}", n);
						}
						self.sg_transfer += 1;
						Ok(())
					},
					Err(e) => {
						error!("Terminate protocol. Could not complete terminate process.\nError: {}", e);
						Err(())
					},
				}
			},
			_ => Ok(()),
		}
	}

	/// Set up command
	pub fn cmd_setup(&mut self, cmdsize: u32, direction: Direction) {
		// Clear the array
		self.cmdidx = 0;
		self.cmdbuf = [0; SG];
		self.databuf = [0; 4096];
		trace!("Command array cleared: {:?}", self.cmdbuf);

		match self.version.stlink {
			// If the STLink version is V1 prepare the array
			1 => {
				self.cmdbuf[0] = 'U' as u8;
				self.cmdbuf[1] = 'S' as u8;
				self.cmdbuf[2] = 'B' as u8;
				self.cmdbuf[3] = 'C' as u8;

				// Write the number of the sg_transfer
				buf_write_u32(&mut self.cmdbuf, 4, self.sg_transfer as u32, true);
				// Write the length of the command
				buf_write_u32(&mut self.cmdbuf, 8, cmdsize, true);

				self.cmdbuf[12] = match direction {
					Direction::In => 0x80,
					Direction::Out => 0x81,
				};
				self.cmdbuf[13] = 0;
				self.cmdbuf[14] = 0xA;
				self.cmdidx = 15;

			},
			// Else returns
			_ => (),
		}
	}

	/// Pushes a new command into the command buffer
	pub fn push_command(&mut self, cmd: u8) {
		self.cmdbuf[self.cmdidx] = cmd;
		self.cmdidx += 1;
	}
}

