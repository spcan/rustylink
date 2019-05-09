//! Send and receive methods of the `Link` struct

use crate::link::enums::{ Cmd };
use crate::link::util::{ buf_write_u32, buf_write_u16 };

use libusb::Direction;

use super::Link;

/// Send and receive (IO) functions
impl<'a> Link<'a> {
	/// Send command and data
	/// `txsize` indicates the amount of bytes to be transfered from the data buffer
	/// `cmdsize` indicates the amount od command bytes to be sent
	/// `terminate` indicates, if this is a STLinkV1 device, if the terminate sequence is executed
	/// Returns `Ok(n)` where `n` is the amount of bytes sent or `Err(())` if there was an error
	pub fn send(&mut self, cmdsize: usize, txsize: usize, terminate: bool) -> Result<usize, ()> {
		use super::super::constants::misc::TIMEOUT::WRITE as WriteTimeout;
		// Send command
		match self.handle.write_bulk(self.tx, &self.cmdbuf[0..cmdsize], WriteTimeout) {
			// Check the amount of bytes transfered
			Ok(n) if n == cmdsize => (),
			Ok(n) => error!("Send protocol. Tried to send {} command bytes, sent {}", cmdsize, n),
			Err(e) => {
				error!("Send protocol. Failure to send command while sending internal buffer.\nError: {}", e);
				return Err(());
			},
		}

		match txsize {
			// If txsize = 0, don't send anything
			0 => Ok(0),
			// If there is data to transmit, send `txsize` bytes from the internal buffer
			_ => match self.handle.write_bulk(self.tx, &self.databuf[0..txsize], WriteTimeout) {
				Ok(n) if n == txsize => Ok(n),
				Ok(n) => {
					error!("Send protocol. Tried to send {} bytes of data, sent {}", txsize, n);

					if terminate { self.terminate()?; }

					Ok(n)
				},
				Err(e) => {
					error!("Send protocol. Failure to send internal buffer.\nError: {}", e);
					Err(())
				},
			}
		}
	}

	/// Send command and receive data
	/// `rxsize` indicates the amount of bytes to be transfered from the data buffer
	/// `cmdsize` indicates the amount od command bytes to be sent
	/// `terminate` indicates, if this is a STLinkV1 device, if the terminate sequence is executed
	/// Returns `Ok(n)` where `n` is the amount of bytes received or `Err(())` if there was an error
	pub fn recv(&mut self, cmdsize: usize, rxsize: usize, terminate: bool) -> Result<usize, ()> {
		use super::super::constants::misc::TIMEOUT::WRITE as WriteTimeout;
		// Receive command
		match self.handle.write_bulk(self.tx, &self.cmdbuf[0..cmdsize], WriteTimeout) {
			// Check the amount of bytes transfered
			Ok(n) if n == cmdsize => (),
			Ok(n) => error!("Receive protocol. Tried to send {} command bytes, sent {}", cmdsize, n),
			Err(e) => {
				error!("Receive protocol. Failure to send command while sending internal buffer.\nError: {}", e);
				return Err(());
			},
		}

		match rxsize {
			// If rxsize = 0, don't receive anything
			0 => Ok(0),
			// If there is data to transmit, send `rxsize` bytes from the internal buffer
			_ => match self.handle.read_bulk(self.rx, &mut self.databuf, WriteTimeout) {
				Ok(n) if n == rxsize => Ok(n),
				Ok(n) => {
					error!("Receive protocol. Tried to receive {} bytes of data, sent {}", rxsize, n);

					if terminate { self.terminate()?; }

					Ok(n)
				},
				Err(e) => {
					error!("Receive protocol. Failure to read into internal buffer.\nError: {}", e);
					Err(())
				},
			}
		}
	}


	/// Quick way to make commands
	pub fn command(&mut self, size: usize, dir: Direction, cmds: Vec<Cmd>) -> Result<usize, ()> {
		self.cmd_setup(size as u32, dir);
		cmds.iter().for_each(|&cmd| match cmd {
			Cmd::Int8(byte) => self.push_command(byte),
			Cmd::Int16(int) => {
				buf_write_u16(&mut self.databuf, self.cmdidx, int, true);
				self.cmdidx += 2;
			},
			Cmd::Int32(int) => {
				buf_write_u32(&mut self.databuf, self.cmdidx, int, true);
				self.cmdidx += 4;
			},
		});

		match dir {
			Direction::In  => self.recv(self.cmdidx, size, true),
			Direction::Out => self.send(self.cmdidx, size, true),
		}
	}
}
