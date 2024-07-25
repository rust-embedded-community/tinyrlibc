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

/// Rust implementation of C library function `strrchr`
#[no_mangle]
pub unsafe extern "C" fn strrchr(haystack: *const CChar, needle: CInt) -> *const CChar {
	let mut last = core::ptr::null();
	for idx in 0.. {
		let ptr = haystack.offset(idx);
		if needle == (*ptr) as CInt {
			last = ptr;
		}
		if (*ptr) == 0 {
			break;
		}
	}
	last
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn strchr_no_match() {
		let haystack = b"hayyystack\0".as_ptr();
		let result = unsafe { strchr(haystack, b'X' as CInt) };
		assert_eq!(result, core::ptr::null());
	}

	#[test]
	fn strchr_null() {
		let haystack = b"hayyystack\0".as_ptr();
		let result = unsafe { strchr(haystack, 0) };
		assert_eq!(result, unsafe { haystack.offset(10) });
	}

	#[test]
	fn strchr_start() {
		let haystack = b"hayyystack\0".as_ptr();
		let result = unsafe { strchr(haystack, b'h' as CInt) };
		assert_eq!(result, haystack);
	}

	#[test]
	fn strchr_middle() {
		let haystack = b"hayyystack\0".as_ptr();
		let result = unsafe { strchr(haystack, b'y' as CInt) };
		assert_eq!(result, unsafe { haystack.offset(2) });
	}

	#[test]
	fn strchr_end() {
		let haystack = b"hayyystack\0".as_ptr();
		let result = unsafe { strchr(haystack, b'k' as CInt) };
		assert_eq!(result, unsafe { haystack.offset(9) });
	}

	#[test]
	fn strrchr_no_match() {
		let haystack = b"hayyystack\0".as_ptr();
		let result = unsafe { strrchr(haystack, b'X' as CInt) };
		assert_eq!(result, core::ptr::null());
	}

	#[test]
	fn strrchr_null() {
		let haystack = b"hayyystack\0".as_ptr();
		let result = unsafe { strrchr(haystack, 0) };
		assert_eq!(result, unsafe { haystack.offset(10) });
	}

	#[test]
	fn strrchr_start() {
		let haystack = b"hayhay\0".as_ptr();
		let result = unsafe { strrchr(haystack, b'h' as CInt) };
		assert_eq!(result, unsafe { haystack.offset(3) });
	}

	#[test]
	fn strrchr_middle() {
		let haystack = b"hayyystack\0".as_ptr();
		let result = unsafe { strrchr(haystack, b'y' as CInt) };
		assert_eq!(result, unsafe { haystack.offset(4) });
	}

	#[test]
	fn strrchr_end() {
		let haystack = b"hayyystack\0".as_ptr();
		let result = unsafe { strrchr(haystack, b'k' as CInt) };
		assert_eq!(result, unsafe { haystack.offset(9) });
	}
}
