//! Rust implementation of C library function `strncmp`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

use core::ffi::{c_char, c_int};

/// Rust implementation of C library function `strncmp`.
///
/// Passing NULL (core::ptr::null()) gives undefined behaviour.
#[cfg_attr(feature = "strncmp", no_mangle)]
pub unsafe extern "C" fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int {
	for i in 0..n as isize {
		let s1_i = s1.offset(i);
		let s2_i = s2.offset(i);

		let val = *s1_i as c_int - *s2_i as c_int;
		if val != 0 || *s1_i == 0 {
			return val;
		}
	}
	0
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn matches() {
		let a = c"123";
		let b = c"1234";
		let result = unsafe { strncmp(a.as_ptr(), b.as_ptr(), 3) };
		// Match!
		assert_eq!(result, 0);
	}

	#[test]
	fn no_match() {
		let a = c"123";
		let b = c"x1234";
		let result = unsafe { strncmp(a.as_ptr(), b.as_ptr(), 3) };
		// No match, first string first
		assert!(result < 0);
	}

	#[test]
	fn no_match2() {
		let a = c"bbbbb";
		let b = c"aaaaa";
		let result = unsafe { strncmp(a.as_ptr(), b.as_ptr(), 3) };
		// No match, second string first
		assert!(result > 0);
	}
}
