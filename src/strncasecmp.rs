//! Rust implementation of C library function `strncasecmp`
//!
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::{CChar, CInt};

/// Rust implementation of C library function `strncasecmp`. Passing NULL
/// (core::ptr::null()) gives undefined behaviour.
#[cfg_attr(feature = "strncasecmp", no_mangle)]
pub unsafe extern "C" fn strncasecmp(s1: *const CChar, s2: *const CChar, n: usize) -> CInt {
	for i in 0..n as isize {
		let s1_i = s1.offset(i);
		let s2_i = s2.offset(i);

		let val = (*s1_i).to_ascii_lowercase() as CInt - (*s2_i).to_ascii_lowercase() as CInt;
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
		let a = b"abc\0";
		let b = b"AbCDEF\0";
		let result = unsafe { strncasecmp(a.as_ptr(), b.as_ptr(), 3) };
		// Match!
		assert_eq!(result, 0);
	}

	#[test]
	fn no_match() {
		let a = b"123\0";
		let b = b"x1234\0";
		let result = unsafe { strncasecmp(a.as_ptr(), b.as_ptr(), 3) };
		// No match, first string first
		assert!(result < 0);
	}

	#[test]
	fn no_match2() {
		let a = b"bbbbb\0";
		let b = b"aaaaa\0";
		let result = unsafe { strncasecmp(a.as_ptr(), b.as_ptr(), 3) };
		// No match, second string first
		assert!(result > 0);
	}
}
