//! Rust implementation of C library function `strstr`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::{CChar, CStringIter};


/// Rust implementation of C library function `strstr`
#[cfg_attr(feature = "strstr", no_mangle)]
pub unsafe extern "C" fn strstr(
	haystack: *const CChar,
	needle: *const CChar,
) -> *const CChar {
	if *needle.offset(0) == 0 {
		return haystack;
	}
	for haystack_trim in (0..).map(|idx| haystack.offset(idx)) {
		if *haystack_trim == 0 {
			break;
		}
		let mut len = 0;
		for (inner_idx, nec) in CStringIter::new(needle).enumerate() {
			let hsc = *haystack_trim.add(inner_idx);
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
		let needle = b"needle\0".as_ptr();
		let haystack = b"haystack\0".as_ptr();
		let result = unsafe { strstr(haystack, needle) };
		assert_eq!(result, core::ptr::null());
	}

	#[test]
	fn start() {
		let needle = b"hay\0".as_ptr();
		let haystack = b"haystack\0".as_ptr();
		let result = unsafe { strstr(haystack, needle) };
		assert_eq!(result, haystack);
	}

	#[test]
	fn middle() {
		let needle = b"yst\0".as_ptr();
		let haystack = b"haystack\0".as_ptr();
		let result = unsafe { strstr(haystack, needle) };
		assert_eq!(result, unsafe { haystack.offset(2) });
	}

	#[test]
	fn end() {
		let needle = b"stack\0".as_ptr();
		let haystack = b"haystack\0".as_ptr();
		let result = unsafe { strstr(haystack, needle) };
		assert_eq!(result, unsafe { haystack.offset(3) });
	}

	#[test]
	fn partial() {
		let needle = b"haystacka\0".as_ptr();
		let haystack = b"haystack\0".as_ptr();
		let result = unsafe { strstr(haystack, needle) };
		assert_eq!(result, core::ptr::null());
	}
}
