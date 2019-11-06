//! Rust implementation of C library function `itoa`
//!
//! This function isn't part of POSIX, or the C standard library, but it is
//! mentioned in K&R, and it useful when writing a `sprintf` implementation.
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::CChar;

#[no_mangle]
/// Formats the given value `i`, with the given radix, into the given buffer (of the given length).
///
/// No prefixes (like 0x or 0b) are generated. Only radix values in the range
/// 2..=16 are supported.
///
/// Returns the number of bytes written on success (not including the null),
/// or -1 if the buffer wasn't large enough.
pub unsafe extern "C" fn itoa(i: i64, s: *mut CChar, s_len: usize, radix: u8) -> i32 {
	let (is_negative, pos_i) = if i < 0 {
		(true, (-i) as u64)
	} else {
		(false, i as u64)
	};

	if is_negative && (s_len > 0) {
		core::ptr::write(s, b'-');
		utoa(pos_i, s.offset(1), s_len - 1, radix)
	} else {
		utoa(pos_i, s, s_len, radix)
	}
}

#[no_mangle]
/// Formats the given value `u`, with the given radix, into the given buffer (of the given length).
///
/// No prefixes (like 0x or 0b) are generated. Only radix values in the range
/// 2..=16 are supported. Negative numbers are not supported.
///
/// Returns the number of bytes written on success (not including the null),
/// or -1 if the buffer wasn't large enough.
pub unsafe extern "C" fn utoa(mut u: u64, s: *mut CChar, s_len: usize, radix: u8) -> i32 {
	let buffer_slice = core::slice::from_raw_parts_mut(s, s_len);

	// Build the number up in buffer s in reverse order
	let mut index = 0usize;
	for slot in buffer_slice.iter_mut() {
		let digit = (u % radix as u64) as u8;
		if digit <= 9 {
			*slot = digit + b'0';
		} else {
			*slot = digit - 10 + b'a';
		}
		index += 1;
		u /= radix as u64;
		if u == 0 {
			break;
		}
	}

	if u != 0 {
		return -1;
	}

	// Null-terminate
	if index < buffer_slice.len() {
		buffer_slice[index] = b'\0';
	}

	// Reverse buffer into correct order
	buffer_slice[0..index].reverse();

	index as i32
}

#[cfg(test)]
mod test {
	use super::{itoa, utoa};
	use crate::strcmp::strcmp;

	#[test]
	fn zero() {
		let mut buf = [b'\0'; 32];
		assert_eq!(unsafe { itoa(0, buf.as_mut_ptr(), buf.len(), 10) }, 1);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"0\0" as *const u8) },
			0
		);
	}

	#[test]
	fn one() {
		let mut buf = [b'\0'; 32];
		assert_eq!(unsafe { itoa(1, buf.as_mut_ptr(), buf.len(), 10) }, 1);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"1\0" as *const u8) },
			0
		);
	}

	#[test]
	fn hundredish() {
		let mut buf = [b'\0'; 32];
		assert_eq!(unsafe { itoa(123, buf.as_mut_ptr(), buf.len(), 10) }, 3);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"123\0" as *const u8) },
			0
		);
	}

	#[test]
	fn too_small() {
		let mut buf = [b'\0'; 1];
		assert_eq!(unsafe { itoa(10, buf.as_mut_ptr(), buf.len(), 10) }, -1);
	}

	#[test]
	fn hex() {
		let mut buf = [b'\0'; 32];
		assert_eq!(
			unsafe { itoa(0xDEADBEEF, buf.as_mut_ptr(), buf.len(), 16) },
			8
		);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"deadbeef\0" as *const u8) },
			0
		);
	}

	#[test]
	fn octal() {
		let mut buf = [b'\0'; 32];
		assert_eq!(unsafe { itoa(0o774, buf.as_mut_ptr(), buf.len(), 8) }, 3);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"774\0" as *const u8) },
			0
		);
	}

	#[test]
	fn binary() {
		let mut buf = [b'\0'; 32];
		assert_eq!(
			unsafe { itoa(0b11100010, buf.as_mut_ptr(), buf.len(), 2) },
			8
		);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"11100010\0" as *const u8) },
			0
		);
	}

	#[test]
	fn negative() {
		let mut buf = [b'\0'; 32];
		unsafe { itoa(-123, buf.as_mut_ptr(), buf.len(), 10) };
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"-123\0" as *const u8) },
			0
		);
	}

	#[test]
	fn unsigned_zero() {
		let mut buf = [b'\0'; 32];
		assert_eq!(unsafe { utoa(0, buf.as_mut_ptr(), buf.len(), 10) }, 1);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"0\0" as *const u8) },
			0
		);
	}

	#[test]
	fn unsigned_one() {
		let mut buf = [b'\0'; 32];
		assert_eq!(unsafe { utoa(1, buf.as_mut_ptr(), buf.len(), 10) }, 1);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"1\0" as *const u8) },
			0
		);
	}

	#[test]
	fn unsigned_hundredish() {
		let mut buf = [b'\0'; 32];
		assert_eq!(unsafe { utoa(123, buf.as_mut_ptr(), buf.len(), 10) }, 3);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"123\0" as *const u8) },
			0
		);
	}

	#[test]
	fn unsigned_too_small() {
		let mut buf = [b'\0'; 1];
		assert_eq!(unsafe { utoa(10, buf.as_mut_ptr(), buf.len(), 10) }, -1);
	}

	#[test]
	fn unsigned_hex() {
		let mut buf = [b'\0'; 32];
		assert_eq!(
			unsafe { utoa(0xDEADBEEF, buf.as_mut_ptr(), buf.len(), 16) },
			8
		);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"deadbeef\0" as *const u8) },
			0
		);
	}

	#[test]
	fn unsigned_octal() {
		let mut buf = [b'\0'; 32];
		assert_eq!(unsafe { utoa(0o774, buf.as_mut_ptr(), buf.len(), 8) }, 3);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"774\0" as *const u8) },
			0
		);
	}

	#[test]
	fn unsigned_binary() {
		let mut buf = [b'\0'; 32];
		assert_eq!(
			unsafe { utoa(0b11100010, buf.as_mut_ptr(), buf.len(), 2) },
			8
		);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"11100010\0" as *const u8) },
			0
		);
	}
}
