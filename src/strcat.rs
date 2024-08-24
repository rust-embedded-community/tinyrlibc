//! Rust implementation of C library function `strcat`
//!
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::CChar;

/// Rust implementation of C library function `strcat`. Passing NULL
/// (core::ptr::null()) gives undefined behaviour.
#[cfg_attr(feature = "strcat", no_mangle)]
pub unsafe extern "C" fn strcat(dest: *mut CChar, src: *const CChar) -> *const CChar {
	crate::strcpy::strcpy(dest.add(crate::strlen::strlen(dest)), src);
	dest
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn simple() {
		let mut dest = *b"hello\0";
		let src = *b" world\0";
		let result = unsafe { strcat(dest.as_mut_ptr(), src.as_ptr()) };
		assert_eq!(
			unsafe { core::slice::from_raw_parts(result, 12) },
			b"hello world\0"
		);
	}
}
