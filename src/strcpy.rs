//! Rust implementation of C library function `strcpy`
//!
//! Copyright (c) Dion Dokter 2022
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::CChar;

/// Rust implementation of C library function `strcpy`. Passing NULL
/// (core::ptr::null()) gives undefined behaviour.
#[no_mangle]
pub unsafe extern "C" fn strcpy(dest: *mut CChar, src: *const CChar) -> *const CChar {
	let mut i = 0isize;
	loop {
		*dest.offset(i) = *src.offset(i);

		if *src.offset(i) == b'\0' {
			break;
		}

		i += 1;
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
		let result = unsafe { strcpy(dest.as_mut_ptr(), src.as_ptr()) };
		assert_eq!(
			unsafe { core::slice::from_raw_parts(result, 6) },
			*b"hi\0def"
		);
	}

	#[test]
	fn two() {
		let src = b"hi\0";
		let mut dest = [0u8; 3];
		let result = unsafe { strcpy(dest.as_mut_ptr(), src.as_ptr()) };
		assert_eq!(unsafe { core::slice::from_raw_parts(result, 3) }, b"hi\0");
	}
}
