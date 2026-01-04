//! Rust implementation of C library function `strchr`
//!
//! Copyright (c) 42 Technology Ltd
//! Licensed under the Blue Oak Model Licence 1.0.0

use core::ffi::{c_char, c_int};

/// Rust implementation of C library function `strrchr`
#[cfg_attr(feature = "strrchr", no_mangle)]
pub unsafe extern "C" fn strrchr(haystack: *const c_char, needle: c_int) -> *const c_char {
	let mut last = core::ptr::null();
	for idx in 0.. {
		let ptr = haystack.offset(idx);
		if needle == (*ptr) as c_int {
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
	fn strrchr_no_match() {
		let haystack = c"hayyystack".as_ptr();
		let result = unsafe { strrchr(haystack, b'X' as c_int) };
		assert_eq!(result, core::ptr::null());
	}

	#[test]
	fn strrchr_null() {
		let haystack = c"hayyystack".as_ptr();
		let result = unsafe { strrchr(haystack, 0) };
		assert_eq!(result, unsafe { haystack.offset(10) });
	}

	#[test]
	fn strrchr_start() {
		let haystack = c"hayhay".as_ptr();
		let result = unsafe { strrchr(haystack, b'h' as c_int) };
		assert_eq!(result, unsafe { haystack.offset(3) });
	}

	#[test]
	fn strrchr_middle() {
		let haystack = c"hayyystack".as_ptr();
		let result = unsafe { strrchr(haystack, b'y' as c_int) };
		assert_eq!(result, unsafe { haystack.offset(4) });
	}

	#[test]
	fn strrchr_end() {
		let haystack = c"hayyystack".as_ptr();
		let result = unsafe { strrchr(haystack, b'k' as c_int) };
		assert_eq!(result, unsafe { haystack.offset(9) });
	}
}
