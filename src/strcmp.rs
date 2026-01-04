//! Rust implementation of C library function `strncmp`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

use core::ffi::{c_char, c_int};

/// Rust implementation of C library function `strcmp`
#[cfg_attr(feature = "strcmp", no_mangle)]
pub unsafe extern "C" fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int {
	for i in 0.. {
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
	fn test1() {
		assert!(unsafe { strcmp(c"Hello".as_ptr(), c"Hello".as_ptr()) } == 0);
	}

	#[test]
	fn test2() {
		assert!(unsafe { strcmp(c"Hello".as_ptr(), c"Hello1".as_ptr()) } < 0);
	}

	#[test]
	fn test3() {
		assert!(unsafe { strcmp(c"Hello1".as_ptr(), c"Hello".as_ptr()) } > 0);
	}

	#[test]
	fn test4() {
		assert!(unsafe { strcmp(c"".as_ptr(), c"Hello".as_ptr()) } < 0);
	}

	#[test]
	fn test5() {
		assert!(unsafe { strcmp(c"Hello".as_ptr(), c"".as_ptr()) } > 0);
	}
}
