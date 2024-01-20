//! Rust implementation of C library function `strcpy`
//!
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::CChar;

/// Rust implementation of C library function `strcpy`. Passing NULL
/// (core::ptr::null()) gives undefined behaviour.
#[cfg_attr(feature = "strcpy", export_name = "strcpy")]
pub unsafe fn strcpy(dest: *mut CChar, src: *const CChar) -> *const CChar {
	let mut i = 0;
	loop {
		*dest.offset(i) = *src.offset(i);
		let c = *dest.offset(i);
		if c == 0 {
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
			unsafe { core::slice::from_raw_parts(result, 5) },
			*b"hi\0de"
		);
	}

	#[test]
	fn two() {
		let src = b"hi\0";
		let mut dest = [0u8; 2]; // no space for null terminator
		let result = unsafe { strcpy(dest.as_mut_ptr(), src.as_ptr()) };
		assert_eq!(unsafe { core::slice::from_raw_parts(result, 2) }, b"hi");
	}
}
