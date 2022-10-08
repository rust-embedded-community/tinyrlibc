//! A tiny C library, written in Rust.
//!
//! See README.md for more details.
//!
//! This file is Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0
//!
//! See each module for its respective licence.

#![cfg_attr(not(test), no_std)]

#[cfg(test)]
#[allow(unused_imports)]
use std as core;

mod abs;
pub use self::abs::abs;

mod strcmp;
pub use self::strcmp::strcmp;

mod strncmp;
pub use self::strncmp::strncmp;

mod strcpy;
pub use self::strcpy::strcpy;

mod strncpy;
pub use self::strncpy::strncpy;

mod strlen;
pub use self::strlen::strlen;

mod strtol;
pub use self::strtol::strtol;

mod strtoul;
pub use self::strtoul::strtoul;

mod strstr;
pub use self::strstr::strstr;

mod strchr;
pub use self::strchr::strchr;

mod atoi;
pub use self::atoi::atoi;

mod itoa;
pub use self::itoa::itoa;

mod snprintf;

/// `long long int`
pub type CLongLong = ::core::ffi::c_longlong;

/// `unsigned long long int`
pub type CULongLong = ::core::ffi::c_ulonglong;

/// `long int`
pub type CLong = ::core::ffi::c_long;

/// `unsigned long int`
pub type CULong = ::core::ffi::c_ulong;

/// `int`
pub type CInt = ::core::ffi::c_int;

/// `unsigned int`
pub type CUInt = ::core::ffi::c_uint;

/// Represents an 8-bit `char`. Rust does not (and will never) support
/// platforms where `char` is not 8-bits long.
pub type CChar = u8;

/// This allows you to iterate a null-terminated string in a relatively simple
/// way.
pub struct CStringIter {
	ptr: *const CChar,
	idx: isize,
}

impl CStringIter {
	/// Create a new iterator from a pointer to a null-terminated string. The
	/// behaviour is undefined if the string is not null-terminated.
	pub fn new(s: *const CChar) -> CStringIter {
		CStringIter { ptr: s, idx: 0 }
	}
}

impl core::iter::Iterator for CStringIter {
	type Item = CChar;
	fn next(&mut self) -> Option<Self::Item> {
		let c = unsafe { *self.ptr.offset(self.idx) };
		if c == 0 {
			None
		} else {
			self.idx += 1;
			Some(c)
		}
	}
}
