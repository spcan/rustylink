//! Utility functions

#[inline]
pub fn parity_u32(x: u32) -> bool {
	let mut y = x;
	y ^= y >> 16;
	y ^= y >> 8;
	y ^= y >> 4;
	y ^= y >> 2;
	y ^= y >> 1;
	y & 1 == 1
}


/// Retrieves `num` bits from `buffer`, starting at the `first` bit,
/// returning the bits in a 32-bit word.  This routine fast-paths reads
/// of little-endian, byte-aligned, 32-bit words.
#[inline]
pub fn buf_get_u32(buffer: &[u8], first: usize, num: usize) -> u32 {
	match (num, first) {
		(32, 0) => {
			(buffer[3] as u32) << 24 |
			(buffer[2] as u32) << 16 |
			(buffer[1] as u32) <<  8 |
			(buffer[0] as u32) <<  0  
		},
		_ => {
			let mut result = 0;
			for i in first..(first+num) {
				if ((buffer[i/8] >> (i%8)) & 1) == 1 {
					result |= 1 << (i - first);
				}
			}
			result
		},
	}
}

#[inline]
pub fn buf_set_u32(buffer: &mut [u8], first: usize, num: usize, value: u32) {
	match (num, first) {
		(32, 0) => {
			buffer[3] = ((value >> 24) & 0xff) as u8;
			buffer[2] = ((value >> 16) & 0xff) as u8;
			buffer[1] = ((value >>  8) & 0xff) as u8;
			buffer[0] = ((value >>  0) & 0xff) as u8;
		},
		_ => for i in first..(first+num) {
			match (value >> (i-first)) & 1 {
				1 => buffer[i/8] |=   1 << (i%8),
				_ => buffer[i/8] &= !(1 << (i%8)),
			}
		}
	}
}

#[inline]
pub fn le_to_h_u64(buf: &[u8]) -> u64 {
	(buf[0] as u64) <<  0 |
	(buf[1] as u64) <<  8 |
	(buf[2] as u64) << 16 |
	(buf[3] as u64) << 24 |
	(buf[4] as u64) << 32 |
	(buf[5] as u64) << 40 |
	(buf[6] as u64) << 48 |
	(buf[7] as u64) << 56
}

#[inline]
pub fn le_to_h_u32(buf: &[u8]) -> u32 {
	(buf[0] as u32) <<  0 |
	(buf[1] as u32) <<  8 |
	(buf[2] as u32) << 16 |
	(buf[3] as u32) << 24  
}

#[inline]
pub fn le_to_h_u16(buf: &[u8]) -> u16 {
	(buf[0] as u16) <<  0 |
	(buf[1] as u16) <<  8  
}

#[inline]
pub fn be_to_h_u64(buf: &[u8]) -> u64 {
	(buf[7] as u64) <<  0 |
	(buf[6] as u64) <<  8 |
	(buf[5] as u64) << 16 |
	(buf[4] as u64) << 24 |
	(buf[3] as u64) << 32 |
	(buf[2] as u64) << 40 |
	(buf[1] as u64) << 48 |
	(buf[0] as u64) << 56
}

#[inline]
pub fn be_to_h_u32(buf: &[u8]) -> u32 {
	(buf[7] as u32) <<  0 |
	(buf[6] as u32) <<  8 |
	(buf[5] as u32) << 16 |
	(buf[4] as u32) << 24  
}

#[inline]
pub fn be_to_h_u16(buf: &[u8]) -> u16 {
	(buf[7] as u16) <<  0 |
	(buf[6] as u16) <<  8  
}

