//! A tiny C library, written in Rust.
//!
//! See README.md for more details.
//!
//! This file is Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0
//!
//! See each module for its respective license.

#![cfg_attr(not(test), no_std)]
#![allow(clippy::missing_safety_doc)]
#![allow(unused_imports)]

// Useful imports
mod ctype;
pub use self::ctype::*;

// Stateless implementations.
// rustfmt will keep these in alphabetical order.
mod abs;
mod itoa;
mod memchr;
mod qsort;
mod rand_r;
mod snprintf;
mod strcat;
mod strchr;
mod strcmp;
mod strcpy;
mod strcspn;
mod strlen;
mod strncasecmp;
mod strncmp;
mod strncpy;
mod strrchr;
mod strspn;
mod strstr;
mod strtol;

// Stateful implementations (which hence are optional).
// rustfmt will keep these in alphabetical order.
#[cfg(feature = "alloc")]
mod malloc;
#[cfg(feature = "rand")]
mod rand;
#[cfg(feature = "signal")]
mod signal;

// Public re-exports.
// rustfmt will keep these in alphabetical order.
#[cfg(feature = "abs")]
pub use self::abs::abs;
#[cfg(feature = "itoa")]
pub use self::itoa::itoa;
#[cfg(feature = "utoa")]
pub use self::itoa::utoa;
#[cfg(feature = "alloc")]
pub use self::malloc::{calloc, free, malloc, realloc};
#[cfg(feature = "memchr")]
pub use self::memchr::memchr;
#[cfg(feature = "qsort")]
pub use self::qsort::qsort;
#[cfg(feature = "rand")]
pub use self::rand::{rand, srand};
#[cfg(feature = "rand_r")]
pub use self::rand_r::{rand_r, RAND_MAX};
#[cfg(feature = "signal")]
pub use self::signal::{abort, raise, signal};
#[cfg(feature = "strcat")]
pub use self::strcat::strcat;
#[cfg(feature = "strchr")]
pub use self::strchr::strchr;
#[cfg(feature = "strcmp")]
pub use self::strcmp::strcmp;
#[cfg(feature = "strcpy")]
pub use self::strcpy::strcpy;
#[cfg(feature = "strcspn")]
pub use self::strcspn::strcspn;
#[cfg(feature = "strlen")]
pub use self::strlen::strlen;
#[cfg(feature = "strncasecmp")]
pub use self::strncasecmp::strncasecmp;
#[cfg(feature = "strncmp")]
pub use self::strncmp::strncmp;
#[cfg(feature = "strncpy")]
pub use self::strncpy::strncpy;
#[cfg(feature = "strrchr")]
pub use self::strrchr::strrchr;
#[cfg(feature = "strspn")]
pub use self::strspn::strspn;
#[cfg(feature = "strstr")]
pub use self::strstr::strstr;
#[cfg(feature = "atoi")]
pub use self::strtol::atoi;
#[cfg(feature = "isalpha")]
pub use self::strtol::isalpha;
#[cfg(feature = "isdigit")]
pub use self::strtol::isdigit;
#[cfg(feature = "isspace")]
pub use self::strtol::isspace;
#[cfg(feature = "isupper")]
pub use self::strtol::isupper;
#[cfg(feature = "strtoimax")]
pub use self::strtol::strtoimax;
#[cfg(feature = "strtol")]
pub use self::strtol::strtol;
#[cfg(feature = "strtoll")]
pub use self::strtol::strtoll;
#[cfg(feature = "strtoul")]
pub use self::strtol::strtoul;
#[cfg(feature = "strtoull")]
pub use self::strtol::strtoull;
#[cfg(feature = "strtoumax")]
pub use self::strtol::strtoumax;
