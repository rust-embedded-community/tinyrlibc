//! Rust implementation of C library function `strcspn`
//!
//! Copyright (c) Ferrous Systems UK Ltd
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::{CChar, CInt};

/// Rust implementation of C library function `strcspn`
#[cfg_attr(feature = "strcspn", no_mangle)]
pub unsafe extern "C" fn strcspn(s: *const CChar, charset: *const CChar) -> usize {
	if s.is_null() {
		return 0;
	}
	if charset.is_null() {
		return 0;
	}

	let s = unsafe { core::ffi::CStr::from_ptr(s.cast()) };

	let charset = unsafe { core::ffi::CStr::from_ptr(charset.cast()) };

	let bytes = s.to_bytes();
	for (idx, b) in bytes.iter().enumerate() {
		if is_c_in_charset(*b, charset) {
			return idx;
		}
	}

	bytes.len()
}

fn is_c_in_charset(c: u8, charset: &core::ffi::CStr) -> bool {
	for b in charset.to_bytes() {
		if c == *b {
			return true;
		}
	}
	false
}

#[cfg(test)]
mod test {
	#[test]
	fn complete() {
		let charset = c"0123456789";
		let s = c"abcdef";
		assert_eq!(
			unsafe { super::strcspn(s.as_ptr().cast(), charset.as_ptr().cast()) },
			6
		);
	}

	#[test]
	fn subset() {
		let charset = c"0123456789";
		let s = c"xyz1";
		assert_eq!(
			unsafe { super::strcspn(s.as_ptr().cast(), charset.as_ptr().cast()) },
			3
		);
	}

	#[test]
	fn none() {
		let charset = c"0123456789";
		let s = c"567";
		assert_eq!(
			unsafe { super::strcspn(s.as_ptr().cast(), charset.as_ptr().cast()) },
			0
		);
	}

	#[test]
	fn empty_charset() {
		let charset = c"";
		let s = c"AABBCCDD";
		assert_eq!(
			unsafe { super::strcspn(s.as_ptr().cast(), charset.as_ptr().cast()) },
			8
		);
	}

	#[test]
	fn empty_string() {
		let charset = c"0123456789";
		let s = c"";
		assert_eq!(
			unsafe { super::strcspn(s.as_ptr().cast(), charset.as_ptr().cast()) },
			0
		);
	}
}

// End of file
