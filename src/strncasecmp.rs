//! Rust implementation of C library function `strncasecmp`
//!
//! Licensed under the Blue Oak Model Licence 1.0.0

use core::ffi::{c_char, c_int};

/// Rust implementation of C library function `strncasecmp`.
///
/// Passing NULL (core::ptr::null()) gives undefined behaviour.
#[cfg_attr(feature = "strncasecmp", no_mangle)]
pub unsafe extern "C" fn strncasecmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int {
	for i in 0..n {
		let s1_i = s1.add(i);
		let s2_i = s2.add(i);

		let c1 = *s1_i as u8;
		let c2 = *s2_i as u8;
		let val = c1.to_ascii_lowercase() as c_int - c2.to_ascii_lowercase() as c_int;
		if val != 0 || c1 == 0 {
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
		let a = c"abc";
		let b = c"AbCDEF";
		let result = unsafe { strncasecmp(a.as_ptr(), b.as_ptr(), 3) };
		// Match!
		assert_eq!(result, 0);
	}

	#[test]
	fn no_match() {
		let a = c"123";
		let b = c"x1234";
		let result = unsafe { strncasecmp(a.as_ptr(), b.as_ptr(), 3) };
		// No match, first string first
		assert!(result < 0);
	}

	#[test]
	fn no_match2() {
		let a = c"bbbbb";
		let b = c"aaaaa";
		let result = unsafe { strncasecmp(a.as_ptr(), b.as_ptr(), 3) };
		// No match, second string first
		assert!(result > 0);
	}
}
