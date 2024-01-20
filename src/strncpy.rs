//! Rust implementation of C library function `strncpy`
//!
//! Copyright (c) Wouter 'Wassasin' Geraedts 2021
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::CChar;

/// Rust implementation of C library function `strncmp`. Passing NULL
/// (core::ptr::null()) gives undefined behaviour.
#[cfg_attr(feature = "strncpy", export_name = "strncpy")]
pub unsafe fn strncpy(dest: *mut CChar, src: *const CChar, count: usize) -> *const CChar {
	let mut i = 0isize;
	while i < count as isize {
		*dest.offset(i) = *src.offset(i);
		let c = *dest.offset(i);
		i += 1;
		if c == 0 {
			break;
		}
	}
	for j in i..count as isize {
		*dest.offset(j) = 0;
	}
	dest
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn short() {
		let src = b"hi\0";
		let mut dest = *b"abcdef"; // no null terminator
		let result = unsafe { strncpy(dest.as_mut_ptr(), src.as_ptr(), 5) };
		assert_eq!(
			unsafe { core::slice::from_raw_parts(result, 5) },
			*b"hi\0\0\0"
		);
	}

	#[test]
	fn two() {
		let src = b"hi\0";
		let mut dest = [0u8; 2]; // no space for null terminator
		let result = unsafe { strncpy(dest.as_mut_ptr(), src.as_ptr(), 2) };
		assert_eq!(unsafe { core::slice::from_raw_parts(result, 2) }, b"hi");
	}
}
