//! Rust implementation of C library function `atoi`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::strtol;

/// Converts a null-terminated string representing a decimal integer, into an
/// integer. No indication of error.
///
/// ```
/// use tinyrlibc::atoi;
/// assert_eq!(unsafe { atoi(b"123".as_ptr()) }, 123);
/// assert_eq!(unsafe { atoi(b"123x".as_ptr()) }, 123);
/// assert_eq!(unsafe { atoi(b"".as_ptr()) }, 0);
/// ```
#[no_mangle]
pub unsafe extern "C" fn atoi(s: *const crate::CChar) -> crate::CInt {
    let result = strtol(s);
    if result > crate::CInt::max_value() {
        crate::CInt::max_value()
    } else if result < crate::CInt::min_value() {
        crate::CInt::min_value()
    } else {
        result as crate::CInt
    }
}
