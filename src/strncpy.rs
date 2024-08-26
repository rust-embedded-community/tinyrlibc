//! Rust implementation of C library function `strncpy`
//!
//! Copyright (c) Wouter 'Wassasin' Geraedts 2021
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::CChar;

/// Rust implementation of C library function `strncmp`.
///
/// Passing NULL (core::ptr::null()) gives undefined behaviour.
#[cfg_attr(feature = "strncpy", no_mangle)]
pub unsafe extern "C" fn strncpy(
	dest: *mut CChar,
	src: *const CChar,
	count: usize,
) -> *const CChar {
	let mut i = 0;
	while i < count {
		let c = *src.add(i);
		*dest.add(i) = c;
		i += 1;
		if c == 0 {
			break;
		}
	}
	for j in i..count {
		*dest.add(j) = 0;
	}
	dest
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn short() {
		let src = b"hi\0";
		// no null terminator
		let mut dest = *b"abcdef";
		// pass in less than full length of dest, to see which bytes get zeroed
		let result = unsafe { strncpy(dest.as_mut_ptr(), src.as_ptr(), 5) };
		// two bytes of data, 3 bytes of zeros (= 5 bytes total), plus one byte unchanged
		assert_eq!(
			unsafe { core::slice::from_raw_parts(result, 6) },
			*b"hi\0\0\0f"
		);
	}

	#[test]
	fn two() {
		let src = b"hello\0";
		let mut dest = [0u8; 2]; // buffer deliberately too small
		let result = unsafe { strncpy(dest.as_mut_ptr(), src.as_ptr(), dest.len()) };
		assert_eq!(unsafe { core::slice::from_raw_parts(result, 2) }, b"he");
	}
}
