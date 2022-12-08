//! Rust implementation of C library function `strtol`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::{CChar, CLong, CStringIter};

/// Rust implementation of C library function `strtol`.
///
/// Takes a null-terminated string and interprets it as a decimal integer.
/// This integer is returned as a `CLong`. Parsing stops when the first
/// non-digit ASCII byte is seen. If no valid ASCII digit bytes are seen, this
/// function returns zero.
#[no_mangle]
pub unsafe extern "C" fn strtol(s: *const CChar) -> CLong {
	let mut result: CLong = 0;
	for c in CStringIter::new(s) {
		if (b'0'..=b'9').contains(&c) {
			result *= 10;
			result += (c - b'0') as CLong;
		} else {
			break;
		}
	}
	result
}

#[cfg(test)]
mod test {
	use super::strtol;

	#[test]
	fn empty() {
		let result = unsafe { strtol(b"\0".as_ptr()) };
		assert_eq!(result, 0);
	}

	#[test]
	fn non_digit() {
		let result = unsafe { strtol(b"1234x\0".as_ptr()) };
		assert_eq!(result, 1234);
	}

	#[test]
	fn bad_number() {
		let result = unsafe { strtol(b"x\0".as_ptr()) };
		assert_eq!(result, 0);
	}

	#[test]
	fn one() {
		let result = unsafe { strtol(b"1\0".as_ptr()) };
		assert_eq!(result, 1);
	}

	#[test]
	fn hundredish() {
		let result = unsafe { strtol(b"123\0".as_ptr()) };
		assert_eq!(result, 123);
	}

	#[test]
	fn big_long() {
		let result = unsafe { strtol(b"2147483647\0".as_ptr()) };
		assert_eq!(result, 2147483647);
	}
}
