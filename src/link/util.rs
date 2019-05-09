//! STLink utilities

/// Read an u32 from the given `buffer` at `offset`.
/// If `little` is `true`, it reads the least significant bit on the first byte.
#[inline]
pub fn buf_read_u32(buffer: &[u8], offset: usize, little: bool) -> u32 {
	if little {
		(buffer[offset+0] as u32) <<  0 |
		(buffer[offset+1] as u32) <<  8 |
		(buffer[offset+2] as u32) << 16 |
		(buffer[offset+3] as u32) << 24  
	} else {
		(buffer[offset+3] as u32) <<  0 |
		(buffer[offset+2] as u32) <<  8 |
		(buffer[offset+1] as u32) << 16 |
		(buffer[offset+0] as u32) << 24  
	}
}

/// Writes `value` to the given `buffer` at `offset`
/// If `little` is true, it writes in little endian mode
#[inline]
pub fn buf_write_u32(buffer: &mut [u8], offset: usize, value: u32, little: bool) {
	if little {
		buffer[offset+0] = (value >>  0) as u8;
		buffer[offset+1] = (value >>  8) as u8;
		buffer[offset+2] = (value >> 16) as u8;
		buffer[offset+3] = (value >> 24) as u8;
	} else {
		buffer[offset+3] = (value >>  0) as u8;
		buffer[offset+2] = (value >>  8) as u8;
		buffer[offset+1] = (value >> 16) as u8;
		buffer[offset+0] = (value >> 24) as u8;
	}
}


/// Read an u32 from the given `buffer` at `offset`.
/// If `little` is `true`, it reads the least significant bit on the first byte.
#[inline]
pub fn buf_read_u16(buffer: &[u8], offset: usize, little: bool) -> u16 {
	if little {
		(buffer[offset+0] as u16) <<  0 |
		(buffer[offset+1] as u16) <<  8  
	} else {
		(buffer[offset+1] as u16) <<  0 |
		(buffer[offset+0] as u16) <<  8  
	}
}

/// Writes `value` to the given `buffer` at `offset`
/// If `little` is true, it writes in little endian mode
#[inline]
pub fn buf_write_u16(buffer: &mut [u8], offset: usize, value: u16, little: bool) {
	if little {
		buffer[offset+0] = (value >>  0) as u8;
		buffer[offset+1] = (value >>  8) as u8;
	} else {
		buffer[offset+1] = (value >>  0) as u8;
		buffer[offset+0] = (value >>  8) as u8;
	}
}
