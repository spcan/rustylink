//! Internal debugger

pub mod elf;
pub mod link;

use std::path::PathBuf;

/// Memory Usage limitation to 512 MB
pub const MAX_MEM_USAGE: usize = 512 * (1024 * 1024);

pub struct Debugger {
	/// ELF file loaded into memory
	pub elfs: Vec<elf::ElfFile>,
	/// ELF UID counter
	euid: usize,

	/// Links to device interfaces
	pub links: Vec<link::NamedLink>,
	/// Link UID counter
	luid: usize,

	/// Free memory available
	mem: usize,
}

impl Debugger {
	/// Create a new debugger
	pub fn new() -> Self {
		Self {
			elfs:  Vec::new(),
			euid: 0,
			links: Vec::new(),
			luid: 0,
			mem: MAX_MEM_USAGE,
		}
	}

	/// Tries to load a new ELF file
	/// Returns the uid of the new ELF file
	pub fn load(&mut self, path: PathBuf) -> Result<usize, ()> {
		match elf::ElfFile::new(self.euid, String::from(format!("elf{}", self.euid)), path, self.mem as u64) {
			Ok(elf) => {
				self.elfs.push(elf.clone());
				self.euid += 1;
				Ok( self.euid - 1)
			},
			_ => Err(())
		}
	}

	/// Tries to load a new ELF file
	/// Returns the uid of the new ELF file
	pub fn connect(&mut self, send: crossbeam::Sender<Vec<u8>>, recv: crossbeam::Receiver<Vec<u8>>) -> Result<usize, ()> {
		self.links.push(link::NamedLink::new(self.luid, String::from(format!("link{}", self.luid)), send, recv));
		self.luid += 1;
		Ok( self.luid - 1 )
	}

	/// Rename an ELF file
	pub fn rename_elf(&mut self, id: usize, name: String) -> Result<&mut Self, ()> {
		match self.elfs.iter_mut().find(|elf| elf.identify(id) ) {
			Some(e) => {
				e.rename(name);
				Ok( self )
			},
			_ => Err(())
		}
	}

	/// Rename Link
	pub fn rename_link(&mut self, id: usize, name: String) -> Result<&mut Self, ()> {
		match self.links.iter_mut().find(|elf| elf.identify(id) ) {
			Some(l) => {
				l.rename(name);
				Ok( self )
			},
			_ => Err(())
		}
	}
}