//! Rust implementation of C library function `strcpy`
//!
//! Licensed under the Blue Oak Model Licence 1.0.0

use core::ffi::c_char;

/// Rust implementation of C library function `strcpy`.
///
/// Passing NULL (core::ptr::null()) gives undefined behaviour.
#[cfg_attr(feature = "strcpy", no_mangle)]
pub unsafe extern "C" fn strcpy(dest: *mut c_char, src: *const c_char) -> *const c_char {
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
		let src = c"hi";
		let mut dest = *b"abcdef"; // no null terminator
		let result = unsafe { strcpy(dest.as_mut_ptr().cast(), src.as_ptr()) };
		assert_eq!(
			unsafe { core::slice::from_raw_parts(result as *const u8, 6) },
			*b"hi\0def"
		);
	}
}
