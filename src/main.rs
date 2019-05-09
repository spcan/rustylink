

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;
extern crate elf;

extern crate libusb;

//extern crate libusb_sys as rawusb;
extern crate crossbeam_channel as crossbeam;

pub mod logging;

pub mod util;
pub mod usb;
#[macro_use]
pub mod common;
pub mod link;

pub mod dbg;

use crate::link::util::{ buf_read_u32, buf_read_u16 };
use std::time::Duration;


fn main() {
	logging::init(log::LevelFilter::Debug);

	let mut usbctx = match libusb::Context::new() {
		Ok(context) => context,
		Err(e) => {
			error!("Could not open a USB context\n{}", e);
			panic!();
		},
	};

	let mut link = match link::link::Link::open_usb(&mut usbctx, 3, link::enums::DebugMode::SWD) {
		Ok(l) => l,
		_ => panic!("Could not get link"),
	};

	println!("Core      ID: 0x{:X}", link.core_id().unwrap());

	//link.jtag_reset(0);


	//std::thread::sleep(Duration::from_secs(2));

	println!("Current mode: {:?}", link.current_mode());

	link.unlock_flash().unwrap();

	link.usb_reset();
	link.halt();

	debug!("GPIOD mode register. Shouldn't be 0x00 : 0x{:X}", link.read_debug_reg(0x4002_0C00 + 0x0C).unwrap());
	debug!("GPIOD mode register. Read raw: 0x{:X}", buf_read_u32( &link.read_mem(0x4002_0C00 + 0x0C, 4).unwrap(), 0, true) );

	std::thread::sleep(Duration::from_secs(1));

	println!("{}", link.read_core_regs().unwrap());
	link.run();

	let mut dbg = dbg::internal::Debugger::new();
	dbg.load( std::path::PathBuf::from("/home/andres/micro_test/target/thumbv7em-none-eabihf/release/micro_test") );


	link.halt();
	debug!("Reading Vector table");

	println!("{:?}", &dbg.elfs[0].vector.data[0..64] );

	let data = link.read_mem(0x800_0000, 64).unwrap();

	for i in 0..64 {
		if data[i] != dbg.elfs[0].vector.data[i] {
			warn!("Read data differs from ELF File");
		}
	}

	println!("New method:\n{:?}", link.read_mem32(0x800_0000, 60).unwrap() );
	link.dump_data(64);

	link.run();

	loop {
		link.status();
		std::thread::sleep(Duration::from_millis(20));
	}
}


fn print_info(device: libusb::Device, desc: libusb::DeviceDescriptor, handle: libusb::DeviceHandle) {
	let languages = match handle.read_languages(Duration::from_secs(1)) {
		Ok(l) => l,
		_ => panic!(),
	};
	println!("Found device {:x}:{:x}", desc.vendor_id(), desc.product_id());
	println!("Manufacturer: {}", handle.read_manufacturer_string(languages[0], &desc, Duration::from_secs(1)).unwrap());
	println!("Product: {}", handle.read_product_string(languages[0], &desc, Duration::from_secs(1)).unwrap());
	let s = handle.read_serial_number_string(languages[0], &desc, Duration::from_secs(1)).unwrap().into_bytes();
	print!("Serialnumber: ", );
	for b in s {
		print!("{:X}:", b);
	}
	print!("\n");
	println!("At speed {} MBps", match device.speed() {
		libusb::Speed::Low => 1.5,
		libusb::Speed::Full => 12.0,
		libusb::Speed::High => 480.0,
		libusb::Speed::Super => 5000.0,
		_ => 0.0,
	});

	let version = desc.device_version();

	println!("STLink {:?}", version);
	println!("STLink {:x}", common::version_to_raw( version ));

	let cfg = match device.config_descriptor(0) {
		Ok(c) => c,
		_ => panic!(),
	};

	for interface in cfg.interfaces() {
		for idesc in interface.descriptors() {
			for endpoint in idesc.endpoint_descriptors() {
				println!("{:?}", endpoint);
			}
		}
	}

}