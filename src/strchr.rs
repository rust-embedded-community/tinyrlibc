//! Rust implementation of C library function `strchr`
//!
//! Copyright (c) 42 Technology Ltd
//! Licensed under the Blue Oak Model Licence 1.0.0

use core::ffi::{c_char, c_int};

/// Rust implementation of C library function `strchr`
#[cfg_attr(feature = "strchr", no_mangle)]
pub unsafe extern "C" fn strchr(haystack: *const c_char, needle: c_int) -> *const c_char {
	for idx in 0.. {
		let ptr = haystack.offset(idx);
		if needle == (*ptr) as c_int {
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
	fn strchr_no_match() {
		let haystack = c"hayyystack".as_ptr();
		let result = unsafe { strchr(haystack, b'X' as c_int) };
		assert_eq!(result, core::ptr::null());
	}

	#[test]
	fn strchr_null() {
		let haystack = c"hayyystack".as_ptr();
		let result = unsafe { strchr(haystack, 0) };
		assert_eq!(result, unsafe { haystack.offset(10) });
	}

	#[test]
	fn strchr_start() {
		let haystack = c"hayyystack".as_ptr();
		let result = unsafe { strchr(haystack, b'h' as c_int) };
		assert_eq!(result, haystack);
	}

	#[test]
	fn strchr_middle() {
		let haystack = c"hayyystack".as_ptr();
		let result = unsafe { strchr(haystack, b'y' as c_int) };
		assert_eq!(result, unsafe { haystack.offset(2) });
	}

	#[test]
	fn strchr_end() {
		let haystack = c"hayyystack".as_ptr();
		let result = unsafe { strchr(haystack, b'k' as c_int) };
		assert_eq!(result, unsafe { haystack.offset(9) });
	}
}
