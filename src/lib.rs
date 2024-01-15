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

#[cfg(feature = "abs")]
mod abs;
#[cfg(feature = "abs")]
pub use self::abs::abs;

#[cfg(feature = "strcmp")]
mod strcmp;
#[cfg(feature = "strcmp")]
pub use self::strcmp::strcmp;

#[cfg(feature = "strncmp")]
mod strncmp;
#[cfg(feature = "strncmp")]
pub use self::strncmp::strncmp;

#[cfg(feature = "strcpy")]
mod strcpy;
#[cfg(feature = "strcpy")]
pub use self::strcpy::strcpy;

#[cfg(feature = "strncpy")]
mod strncpy;
#[cfg(feature = "strncpy")]
pub use self::strncpy::strncpy;

#[cfg(feature = "strlen")]
mod strlen;
#[cfg(feature = "strlen")]
pub use self::strlen::strlen;

#[cfg(feature = "strtol")]
mod strtol;
#[cfg(feature = "strtol")]
pub use self::strtol::strtol;

#[cfg(feature = "strtoul")]
mod strtoul;
#[cfg(feature = "strtoul")]
pub use self::strtoul::strtoul;

#[cfg(feature = "strstr")]
mod strstr;
#[cfg(feature = "strstr")]
pub use self::strstr::strstr;

#[cfg(feature = "strchr")]
mod strchr;
#[cfg(feature = "strchr")]
pub use self::strchr::strchr;

#[cfg(any(feature = "atoi", feature = "atol", feature = "atoll"))]
mod atoi;
#[cfg(feature = "atoi")]
pub use self::atoi::atoi;
#[cfg(feature = "atol")]
pub use self::atoi::atol;
#[cfg(feature = "atoll")]
pub use self::atoi::atoll;

#[cfg(feature = "itoa")]
mod itoa;
#[cfg(feature = "itoa")]
pub use self::itoa::itoa;

#[cfg(feature = "snprintf")]
mod snprintf;

#[cfg(feature = "qsort")]
mod qsort;
#[cfg(feature = "qsort")]
pub use self::qsort::qsort;

#[cfg(feature = "qsort_r")]
mod qsort_r;
#[cfg(feature = "qsort_r")]
pub use self::qsort_r::qsort_r;

#[cfg(feature = "atof")]
mod atof;
#[cfg(feature = "atof")]
pub use self::atof::atof;

#[cfg(feature = "strtod")]
mod strtod;
#[cfg(feature = "strtod")]
pub use self::strtod::strtod;

#[cfg(feature = "strtof")]
mod strtof;
#[cfg(feature = "strtof")]
pub use self::strtof::strtof;

#[cfg(feature = "isspace")]
mod isspace;
#[cfg(feature = "isspace")]
pub use self::isspace::isspace;

#[cfg(feature = "isdigit")]
mod isdigit;
#[cfg(feature = "isdigit")]
pub use self::isdigit::isdigit;

#[cfg(feature = "errno")]
mod errno;
#[cfg(feature = "errno")]
pub use self::errno::{errno, set_errno};

#[cfg(feature = "memchr")]
mod memchr;
#[cfg(feature = "memchr")]
pub use self::memchr::memchr;

/// `void`
pub type CVoid = ::core::ffi::c_void;

/// `size_t`
pub type CSizeT = usize;

/// `double`
pub type CDouble = core::ffi::c_double;

/// `float`
pub type CFloat = core::ffi::c_float;

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
