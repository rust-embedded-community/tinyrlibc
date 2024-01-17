//! Rust implementation of C library function `strncmp`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::{CChar, CInt};

/// Rust implementation of C library function `strcmp`
#[no_mangle]
#[cfg(feature = "strcmp")]
pub unsafe extern "C" fn strcmp(s1: *const CChar, s2: *const CChar) -> CInt {
	r_strcmp(s1, s2)
}

pub unsafe fn r_strcmp(s1: *const CChar, s2: *const CChar) -> CInt {
	for i in 0.. {
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
	fn test1() {
		assert!(unsafe { r_strcmp(b"Hello\0" as *const CChar, b"Hello\0" as *const CChar) } == 0);
	}

	#[test]
	fn test2() {
		assert!(unsafe { r_strcmp(b"Hello\0" as *const CChar, b"Hello1\0" as *const CChar) } < 0);
	}

	#[test]
	fn test3() {
		assert!(unsafe { r_strcmp(b"Hello1\0" as *const CChar, b"Hello\0" as *const CChar) } > 0);
	}

	#[test]
	fn test4() {
		assert!(unsafe { r_strcmp(b"\0" as *const CChar, b"Hello\0" as *const CChar) } < 0);
	}

	#[test]
	fn test5() {
		assert!(unsafe { r_strcmp(b"Hello\0" as *const CChar, b"\0" as *const CChar) } > 0);
	}
}
