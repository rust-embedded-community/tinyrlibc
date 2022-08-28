//! Rust implementation of C library function `strtoul`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::{CChar, CStringIter, CULong};

/// Rust implementation of C library function `strtoul`.
///
/// Takes a null-terminated string and interprets it as a decimal integer.
/// This integer is returned as a `CULong`. Parsing stops when the first
/// non-digit ASCII byte is seen. If no valid ASCII digit bytes are seen, this
/// function returns zero.
#[no_mangle]
pub unsafe extern "C" fn strtoul(s: *const CChar) -> CULong {
	let mut result: CULong = 0;
	for c in CStringIter::new(s) {
		if c >= b'0' && c <= b'9' {
			result *= 10;
			result += (c - b'0') as CULong;
		} else {
			break;
		}
	}
	result
}

#[cfg(test)]
mod test {
	use super::strtoul;

	#[test]
	fn empty() {
		let result = unsafe { strtoul(b"\0".as_ptr()) };
		assert_eq!(result, 0);
	}

	#[test]
	fn non_digit() {
		let result = unsafe { strtoul(b"1234x\0".as_ptr()) };
		assert_eq!(result, 1234);
	}

	#[test]
	fn bad_number() {
		let result = unsafe { strtoul(b"x\0".as_ptr()) };
		assert_eq!(result, 0);
	}

	#[test]
	fn one() {
		let result = unsafe { strtoul(b"1\0".as_ptr()) };
		assert_eq!(result, 1);
	}

	#[test]
	fn hundredish() {
		let result = unsafe { strtoul(b"123\0".as_ptr()) };
		assert_eq!(result, 123);
	}

	#[test]
	fn big_long() {
		let result = unsafe { strtoul(b"2147483647\0".as_ptr()) };
		assert_eq!(result, 2147483647);
	}
}
