//! Common


pub fn write_u32(buffer: &mut [u8], ui: u32, swap: bool) {
	if swap {
		buffer[0] = (ui >> 24) as u8;
		buffer[1] = (ui >> 16) as u8;
		buffer[2] = (ui >>  8) as u8;
		buffer[3] = (ui >>  0) as u8;
	} else {
		buffer[0] = (ui >>  0) as u8;
		buffer[1] = (ui >>  8) as u8;
		buffer[2] = (ui >> 16) as u8;
		buffer[3] = (ui >> 24) as u8;
	}
}
/*
pub fn write_u16(buffer: &mut [u8], ui: u16, swap: bool) {
	if swap {
		buffer[0] = (ui >> 8) as u8;
		buffer[1] = (ui >> 0) as u8;
	} else {
		buffer[0] = (ui >> 0) as u8;
		buffer[1] = (ui >> 8) as u8;
	}
}
*/
pub fn version_to_raw(version: libusb::Version) -> usize {
	((version.major() as usize) << 8) | ((version.minor() as usize) << 4) | version.sub_minor() as usize
}


#[macro_export]
macro_rules! mask {
	( $x:expr ) => {
		(1u32 << $x) - 1;
	};
}
