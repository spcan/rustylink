//! ELF files module for the internal debugger

use std::path::PathBuf;
use std::io::Read;

use elf;

#[derive(Debug, Clone)]
pub struct ElfFile {
	id: usize,
	name: String,
	path: PathBuf,

	pub vector: SectionInfo,
	pub text: SectionInfo,
	pub rodata: SectionInfo,
	pub bss: SectionInfo,
	pub data: SectionInfo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionInfo {
	pub range: std::ops::Range<usize>,
	pub data: Vec<u8>,
	pub address: usize,
}

impl std::default::Default for SectionInfo {
	fn default() -> Self {
		Self {
			range: (0..0),
			data: Vec::new(),
			address: 0,
		}
	}
}

impl ElfFile {
	/// Read and Load a new ELF file
	pub fn new(id: usize, name: String, path: PathBuf, max_size: u64) -> Result<Self, ()> {
		let mut new = Self {
			id,
			name,
			path: path.clone(),
			vector: Default::default(),
			text: Default::default(),
			bss: Default::default(),
			data: Default::default(),
			rodata: Default::default(),
		};

		let filedata = match elf::File::open_path(path.clone()) {
			Ok(f) => f,
			Err(e) => {
				error!("Could not open file {:?}.\nError: {:?}", path, e);
				return Err(());
			},
		};

		match filedata.get_section(".vector_table") {
			Some(section) => {
				info!("ELF File '.vector_table' section.");
				info!("  name: {}", section.shdr.name);
				info!("  type: {}", section.shdr.shtype);
				info!("  link-address: 0x{:X}", section.shdr.addr);
				info!("  size: {} kB", section.shdr.size as f32 / 1024.0);
				info!("  offset: {}", section.shdr.offset);

				new.vector.range = section.shdr.offset as usize..(section.shdr.offset + section.shdr.size) as usize;
				new.vector.data = section.data.clone();
				new.vector.address = section.shdr.addr as usize;
			},
			None => {
				error!("Could not find a '.vector_table' section.");
				return Err(());
			},
		}

		match filedata.get_section(".text") {
			Some(section) => {
				info!("ELF File '.text' section.");
				info!("  name: {}", section.shdr.name);
				info!("  type: {}", section.shdr.shtype);
				info!("  link-address: 0x{:X}", section.shdr.addr);
				info!("  size: {} kB", section.shdr.size as f32 / 1024.0);

				new.text.range = section.shdr.offset as usize..(section.shdr.offset + section.shdr.size) as usize;
				new.text.data = section.data.clone();
				new.text.address = section.shdr.addr as usize;
			},
			None => {
				error!("Could not find a '.text' section.");
				return Err(());
			},
		}

		match filedata.get_section(".bss") {
			Some(section) => {
				info!("ELF File '.bss' section.");
				info!("  name: {}", section.shdr.name);
				info!("  type: {}", section.shdr.shtype);
				info!("  link-address: 0x{:X}", section.shdr.addr);
				info!("  size: {} kB", section.shdr.size as f32 / 1024.0);

				new.bss.range = section.shdr.offset as usize..(section.shdr.offset + section.shdr.size) as usize;
				new.bss.data = section.data.clone();
				new.bss.address = section.shdr.addr as usize;
			},
			None => {
				error!("Could not find a '.bss' section.");
				return Err(());
			},
		}


		match filedata.get_section(".data") {
			Some(section) => {
				info!("ELF File '.data' section.");
				info!("  name: {}", section.shdr.name);
				info!("  type: {}", section.shdr.shtype);
				info!("  link-address: 0x{:X}", section.shdr.addr);
				info!("  size: {} kB", section.shdr.size as f32 / 1024.0);

				new.data.range = section.shdr.offset as usize..(section.shdr.offset + section.shdr.size) as usize;
				new.data.data = section.data.clone();
				new.data.address = section.shdr.addr as usize;
			},
			None => {
				error!("Could not find a '.data' section.");
				return Err(());
			},
		}

		match filedata.get_section(".rodata") {
			Some(section) => {
				info!("ELF File '.rodata' section.");
				info!("  name: {}", section.shdr.name);
				info!("  type: {}", section.shdr.shtype);
				info!("  link-address: 0x{:X}", section.shdr.addr);
				info!("  size: {} kB", section.shdr.size as f32 / 1024.0);

				new.rodata.range = section.shdr.offset as usize..(section.shdr.offset + section.shdr.size) as usize;
				new.rodata.data = section.data.clone();
				new.rodata.address = section.shdr.addr as usize;
			},
			None => {
				error!("Could not find a '.rodata' section.");
				return Err(());
			},
		}

		info!("All ELF sections");
		for s in filedata.sections {
			info!("Section {}\n  size: {} kB\n  link-address: 0x{:X}", s.shdr.name, s.shdr.size as f32 / 1024.0, s.shdr.addr);
		}

		Ok( new )
	}

	/// Rename the ELF file (internal rename, for user handling)
	/// Refer to UI
	pub fn rename(&mut self, name: String) {
		self.name = name;
	}

	/// Returns if it answers to a given ID
	pub fn identify(&self, id: usize) -> bool {
		self.id == id
	}
}

impl std::default::Default for ElfFile {
	fn default() -> Self {
		Self {
			id: 0,
			name: String::from(""),
			path: Default::default(),

			vector: Default::default(),
			text: Default::default(),
			bss: Default::default(),
			data: Default::default(),
			rodata: Default::default(),
		}
	}
}