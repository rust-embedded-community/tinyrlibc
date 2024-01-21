//! Rust implementation of C library function `strchr`
//!
//! Copyright (c) 42 Technology Ltd
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::{CChar, CInt};

/// Rust implementation of C library function `strchr`
#[cfg_attr(feature = "strchr", no_mangle)]
pub unsafe extern "C" fn strchr(haystack: *const CChar, needle: CInt) -> *const CChar {
	for idx in 0.. {
		let ptr = haystack.offset(idx);
		if needle == (*ptr) as CInt {
			return ptr;
		}
		if (*ptr) == 0 {
			break;
		}
	}
	core::ptr::null()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn no_match() {
		let haystack = b"haystack\0".as_ptr();
		let result = unsafe { strchr(haystack, b'X' as CInt) };
		assert_eq!(result, core::ptr::null());
	}

	#[test]
	fn null() {
		let haystack = b"haystack\0".as_ptr();
		let result = unsafe { strchr(haystack, 0) };
		assert_eq!(result, unsafe { haystack.offset(8) });
	}

	#[test]
	fn start() {
		let haystack = b"haystack\0".as_ptr();
		let result = unsafe { strchr(haystack, b'h' as CInt) };
		assert_eq!(result, haystack);
	}

	#[test]
	fn middle() {
		let haystack = b"haystack\0".as_ptr();
		let result = unsafe { strchr(haystack, b'y' as CInt) };
		assert_eq!(result, unsafe { haystack.offset(2) });
	}

	#[test]
	fn end() {
		let haystack = b"haystack\0".as_ptr();
		let result = unsafe { strchr(haystack, b'k' as CInt) };
		assert_eq!(result, unsafe { haystack.offset(7) });
	}
}
