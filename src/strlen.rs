//! Rust implementation of C library function `strlen`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

use core::ffi::c_char;

/// Rust implementation of C library function `strlen`
#[cfg_attr(feature = "strlen", no_mangle)]
pub unsafe extern "C" fn strlen(mut s: *const c_char) -> usize {
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
		assert_eq!(unsafe { strlen(c"Hello".as_ptr()) }, 5);
	}

	#[test]
	fn test2() {
		assert_eq!(unsafe { strlen(c"".as_ptr()) }, 0);
	}

	#[test]
	fn test3() {
		assert_eq!(unsafe { strlen(c"X".as_ptr()) }, 1);
	}
}
