//! Rust implementation of C library function `strlen`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::CChar;

/// Rust implementation of C library function `strlen`
#[cfg(feature = "strlen")]
#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const CChar) -> usize {
	r_strlen(s)
}

pub unsafe fn r_strlen(mut s: *const CChar) -> usize {
	let mut result = 0;
	while *s != 0 {
		s = s.offset(1);
		result += 1;
	}
	result
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test1() {
		assert_eq!(unsafe { r_strlen(b"Hello\0" as *const CChar) }, 5);
	}

	#[test]
	fn test2() {
		assert_eq!(unsafe { r_strlen(b"\0" as *const CChar) }, 0);
	}

	#[test]
	fn test3() {
		assert_eq!(unsafe { r_strlen(b"X\0" as *const CChar) }, 1);
	}
}
