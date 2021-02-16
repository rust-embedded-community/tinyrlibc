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

mod strcmp;
pub use self::strcmp::strcmp;

mod strncmp;
pub use self::strncmp::strncmp;

mod strncpy;
pub use self::strncpy::strncpy;

mod strlen;
pub use self::strlen::strlen;

mod strtol;
pub use self::strtol::strtol;

mod strstr;
pub use self::strstr::strstr;

mod strchr;
pub use self::strchr::strchr;

mod atoi;
pub use self::atoi::atoi;

mod itoa;
pub use self::itoa::itoa;

mod snprintf;

/// `long long int` is always 64-bits long.
pub type CLongLong = i64;

/// `unsigned long long int` is always 64-bits long.
pub type CULongLong = i64;

#[cfg(feature = "lp64")]
/// The `lp64` feature means we assume `long int` is 64-bits.
pub type CLong = i64;

#[cfg(feature = "lp64")]
/// The `lp64` feature means we assume `unsigned long int` is 64-bits.
pub type CULong = u64;

#[cfg(not(feature = "lp64"))]
/// We assume `long int` is 32-bits.
pub type CLong = i32;

#[cfg(not(feature = "lp64"))]
/// We assume `unsigned long int` is 32-bits.
pub type CULong = u32;

#[cfg(feature = "lp32")]
/// The `lp32` feature means we assume `int` is 16-bits.
pub type CInt = i16;

#[cfg(feature = "lp32")]
/// The `lp32` feature means we assume `unsigned int` is 16-bits.
pub type CUInt = u16;

#[cfg(not(feature = "lp32"))]
/// We assume `int` is 32-bits.
pub type CInt = i32;

#[cfg(not(feature = "lp32"))]
/// We assume `unsigned int` is 32-bits.
pub type CUInt = u32;

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
