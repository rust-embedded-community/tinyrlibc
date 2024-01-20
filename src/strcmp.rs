//! Rust implementation of C library function `strncmp`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::{CChar, CInt};

/// Rust implementation of C library function `strcmp`
#[cfg_attr(feature = "strcmp", no_mangle)]
pub unsafe fn strcmp(s1: *const CChar, s2: *const CChar) -> CInt {
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
		assert!(unsafe { strcmp(b"Hello\0" as *const CChar, b"Hello\0" as *const CChar) } == 0);
	}

	#[test]
	fn test2() {
		assert!(unsafe { strcmp(b"Hello\0" as *const CChar, b"Hello1\0" as *const CChar) } < 0);
	}

	#[test]
	fn test3() {
		assert!(unsafe { strcmp(b"Hello1\0" as *const CChar, b"Hello\0" as *const CChar) } > 0);
	}

	#[test]
	fn test4() {
		assert!(unsafe { strcmp(b"\0" as *const CChar, b"Hello\0" as *const CChar) } < 0);
	}

	#[test]
	fn test5() {
		assert!(unsafe { strcmp(b"Hello\0" as *const CChar, b"\0" as *const CChar) } > 0);
	}
}
