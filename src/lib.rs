//! A tiny C library, written in Rust.
//!
//! See README.md for more details.
//!
//! This file is Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0
//!
//! See each module for its respective licence.

#![cfg_attr(not(test), no_std)]
#![allow(clippy::missing_safety_doc)]

#[cfg(test)]
#[allow(unused_imports)]
use std as core;

mod errno;
pub use self::errno::*;

mod itoa;
pub use self::itoa::*;

mod abs;
pub use self::abs::*;

mod strcmp;
pub use self::strcmp::*;

mod strncmp;
pub use self::strncmp::*;

mod strcpy;
pub use self::strcpy::*;

mod strncpy;
pub use self::strncpy::*;

mod strlen;
pub use self::strlen::*;

mod strtol;
pub use self::strtol::*;

mod strstr;
pub use self::strstr::*;

mod strchr;
pub use self::strchr::*;

mod snprintf;

mod ctype;
pub use self::ctype::*;
