//! Rust implementation of C library function `strstr`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

use core::ffi::c_char;

/// Rust implementation of C library function `strstr`
#[cfg_attr(feature = "strstr", no_mangle)]
pub unsafe extern "C" fn strstr(haystack: *const c_char, needle: *const c_char) -> *const c_char {
	if *needle.offset(0) == 0 {
		return haystack;
	}
	for haystack_trim in (0..).map(|idx| haystack.offset(idx)) {
		if *haystack_trim == 0 {
			break;
		}
		let mut len = 0;
		loop {
			let nec = unsafe { needle.offset(len).read() };
			let hsc = unsafe { haystack_trim.offset(len).read() };
			if nec == 0 {
				break;
			}
			if hsc != nec {
				break;
			}
			len += 1;
		}

		if *needle.offset(len) == 0 {
			return haystack_trim;
		}
	}
	core::ptr::null()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn no_match() {
		let needle = c"needle".as_ptr();
		let haystack = c"haystack".as_ptr();
		let result = unsafe { strstr(haystack, needle) };
		assert_eq!(result, core::ptr::null());
	}

	#[test]
	fn start() {
		let needle = c"hay".as_ptr();
		let haystack = c"haystack".as_ptr();
		let result = unsafe { strstr(haystack, needle) };
		assert_eq!(result, haystack);
	}

	#[test]
	fn middle() {
		let needle = c"yst".as_ptr();
		let haystack = c"haystack".as_ptr();
		let result = unsafe { strstr(haystack, needle) };
		assert_eq!(result, unsafe { haystack.offset(2) });
	}

	#[test]
	fn end() {
		let needle = c"stack".as_ptr();
		let haystack = c"haystack".as_ptr();
		let result = unsafe { strstr(haystack, needle) };
		assert_eq!(result, unsafe { haystack.offset(3) });
	}

	#[test]
	fn partial() {
		let needle = c"haystacka".as_ptr();
		let haystack = c"haystack".as_ptr();
		let result = unsafe { strstr(haystack, needle) };
		assert_eq!(result, core::ptr::null());
	}
}
