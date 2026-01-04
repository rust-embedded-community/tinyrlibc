//! Rust implementation of C library function `memchr`
//!
//! Licensed under the Blue Oak Model Licence 1.0.0

use core::ffi::{c_char, c_int, c_void};

/// Rust implementation of C library function `memchr`
#[cfg_attr(feature = "memchr", no_mangle)]
pub unsafe extern "C" fn memchr(s: *const c_void, c: c_int, n: usize) -> *const c_void {
	let s = s as *const c_char;
	for i in 0..n {
		if *s.add(i) as c_int == c {
			return s.add(i) as *const c_void;
		}
	}
	core::ptr::null()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn null() {
		unsafe { assert_eq!(memchr(core::ptr::null(), 0, 0), core::ptr::null()) };
	}

	#[test]
	fn normal() {
		let s = b"hello world";
		assert_eq!(
			unsafe { memchr(s.as_ptr() as *const c_void, b'w' as c_int, s.len() as usize) },
			unsafe { s.as_ptr().offset(6) } as *const c_void
		);
	}

	#[test]
	fn not_found() {
		let s = b"hello world";
		assert_eq!(
			unsafe { memchr(s.as_ptr() as *const c_void, b'x' as c_int, s.len() as usize) },
			core::ptr::null()
		)
	}
}
