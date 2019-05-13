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

mod strlen;
pub use self::strlen::strlen;

mod strtol;
pub use self::strtol::strtol;

mod strstr;
pub use self::strstr::strstr;

mod atoi;
pub use self::atoi::atoi;

// TODO: Add cfg defines / work these out for platforms other than armv6/7/8m

pub type CLongLong = i64;
pub type CLong = i32;
pub type CInt = i32;
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
