//! Rust implementation of C library function `strncmp`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::{CChar, CInt};

/// Rust implementation of C library function `strncmp`. Passing NULL
/// (core::ptr::null()) gives undefined behaviour.
#[cfg(feature = "strncmp")]
#[no_mangle]
pub unsafe extern "C" fn strncmp(s1: *const CChar, s2: *const CChar, n: usize) -> crate::CInt {
    r_strncmp(s1, s2, n)
}

pub(crate) unsafe fn r_strncmp(s1: *const CChar, s2: *const CChar, n: usize) -> crate::CInt {
	for i in 0..n as isize {
		let s1_i = s1.offset(i);
		let s2_i = s2.offset(i);

		let val = *s1_i as CInt - *s2_i as CInt;
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
		let a = b"123\0";
		let b = b"1234\0";
		let result = unsafe { r_strncmp(a.as_ptr(), b.as_ptr(), 3) };
		// Match!
		assert_eq!(result, 0);
	}

	#[test]
	fn no_match() {
		let a = b"123\0";
		let b = b"x1234\0";
		let result = unsafe { r_strncmp(a.as_ptr(), b.as_ptr(), 3) };
		// No match, first string first
		assert!(result < 0);
	}

	#[test]
	fn no_match2() {
		let a = b"bbbbb\0";
		let b = b"aaaaa\0";
		let result = unsafe { r_strncmp(a.as_ptr(), b.as_ptr(), 3) };
		// No match, second string first
		assert!(result > 0);
	}
}
