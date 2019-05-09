//! USB utilities

pub const STLINK_USB_VID_ST               : u32 = 0x0483;
pub const STLINK_USB_PID_STLINK           : u32 = 0x3744;
pub const STLINK_USB_PID_STLINK_32L       : u32 = 0x3748;
pub const STLINK_USB_PID_STLINK_32L_AUDIO : u32 = 0x374a;
pub const STLINK_USB_PID_STLINK_NUCLEO    : u32 = 0x374b;

pub const STLINK_SG_SIZE  : usize = 31;
pub const STLINK_CMD_SIZE : usize = 16;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Endpoint {
    pub config: u8,
    pub iface: u8,
    pub setting: u8,
    pub address: u8
}


/// Analize the USB ports for an On-board STLink
pub fn search<'a>(ctx: &'a libusb::Context) -> Option<(libusb::Device<'a>, libusb::DeviceDescriptor, libusb::DeviceHandle<'a>)> {


	let devices = match ctx.devices() {
		Ok(d) => d,
		Err(e) => {
			error!("USB Error: Could not generate USB devices list.\nError: {}", e);
			return None;
		},
	};

	for device in devices.iter() {
		let desc = match device.device_descriptor() {
			Ok(d) => d,
			Err(e) => {
				error!("USB Error: Could not get device descriptor.\nError: {}", e);
				continue;
			},
		};

		match desc.vendor_id() {
			0x0483 => (),
			_ => continue,
		}

		match desc.product_id() {
			0x3744 | 0x3748 | 0x374a | 0x374b => match device.open() {
				Ok(handle) => {
					info!("Found STLink Device");
					return Some((device, desc, handle));
				},
				Err(e) => {
					error!("USB Error: Could not open device.\nError: {}", e);
					continue;
				},
			},
			_ => warn!("USB Warning: Found a ST device that is not an on board STLink"),

		}
	}

	warn!(target: "critical", "Could not find STLINK device.");

	None
}