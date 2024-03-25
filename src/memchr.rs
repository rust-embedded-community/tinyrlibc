//! Rust implementation of C library function `memchr`
//!
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::{CChar, CInt, CSizeT, CVoid};

/// Rust implementation of C library function `memchr`
#[cfg_attr(feature = "memchr", no_mangle)]
pub unsafe extern "C" fn memchr(s: *const CVoid, c: CInt, n: CSizeT) -> *const CVoid {
	let s = s as *const CChar;
	for i in 0..n {
		if *s.offset(i as isize) as CInt == c {
			return s.offset(i as isize) as *const CVoid;
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
			unsafe { memchr(s.as_ptr() as *const CVoid, b'w' as CInt, s.len() as CSizeT) },
			unsafe { s.as_ptr().offset(6) } as *const CVoid
		);
	}

	#[test]
	fn not_found() {
		let s = b"hello world";
		assert_eq!(
			unsafe { memchr(s.as_ptr() as *const CVoid, b'x' as CInt, s.len() as CSizeT) },
			core::ptr::null()
		)
	}
}
