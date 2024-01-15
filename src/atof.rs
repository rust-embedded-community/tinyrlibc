//! Rust implementation of C library function `atof`
//!
//! Original code from the `c-ward` project.
//! Licensed under the MIT license.

use crate::{CChar, CDouble, strtod};

#[no_mangle]
pub unsafe extern "C" fn atof(nptr: *const CChar) -> CDouble {
    strtod(nptr, core::ptr::null_mut())
}

#[cfg(test)]
mod test {
    #[test]
    fn positive() {
        let result = unsafe { super::atof(b"123.456\0".as_ptr()) };
        assert_eq!(result, 123.456);
    }

    #[test]
    fn negative() {
        let result = unsafe { super::atof(b"-123.456\0".as_ptr()) };
        assert_eq!(result, -123.456);
    }

    #[test]
    fn zero() {
        let result = unsafe { super::atof(b"0\0".as_ptr()) };
        assert_eq!(result, 0.0);
    }

    #[test]
    fn nan() {
        let result = unsafe { super::atof(b"NaN\0".as_ptr()) };
        assert!(result.is_nan());
    }

    #[test]
    fn inf() {
        let result = unsafe { super::atof(b"Inf\0".as_ptr()) };
        assert!(result.is_infinite());
    }

    #[test]
    fn neg_inf() {
        let result = unsafe { super::atof(b"-Inf\0".as_ptr()) };
        assert!(result.is_infinite());
        assert!(result.is_sign_negative());
    }

    #[test]
    fn empty() {
        let result = unsafe { super::atof(b"\0".as_ptr()) };
        assert!(result == 0.0 );
    }

    #[test]
    fn positive_scientific() {
        let result = unsafe { super::atof(b"1.23456e2\0".as_ptr()) };
        assert_eq!(result, 123.456);
    }

    #[test]
    fn negative_scientific() {
        let result = unsafe { super::atof(b"1.23456e-2\0".as_ptr()) };
        assert_eq!(result, 0.0123456);
    }

    #[test]
    fn positive_overflow() {
        let result = unsafe { super::atof(b"1e10000\0".as_ptr()) };
        assert!(result.is_infinite());
    }

    #[test]
    fn negative_overflow() {
        let result = unsafe { super::atof(b"-1e10000\0".as_ptr()) };
        assert!(result.is_infinite());
        assert!(result.is_sign_negative());
    }

    #[test]
    fn leading_whitespace() {
        let result = unsafe { super::atof(b" \t\n\r123.456\0".as_ptr()) };
        assert_eq!(result, 123.456);
    }

    #[test]
    fn trailing_whitespace() {
        let result = unsafe { super::atof(b"123.456 \t\n\r\0".as_ptr()) };
        assert_eq!(result, 123.456);
    }

    #[test]
    fn leading_plus() {
        let result = unsafe { super::atof(b"+123.456\0".as_ptr()) };
        assert_eq!(result, 123.456);
    }

    #[test]
    fn leading_minus() {
        let result = unsafe { super::atof(b"-123.456\0".as_ptr()) };
        assert_eq!(result, -123.456);
    }

    #[test]
    fn leading_plus_nan() {
        let result = unsafe { super::atof(b"+NaN\0".as_ptr()) };
        assert!(result.is_nan());
    }

    #[test]
    fn leading_minus_nan() {
        let result = unsafe { super::atof(b"-NaN\0".as_ptr()) };
        assert!(result.is_nan());
    }

    #[test]
    fn leading_plus_inf() {
        let result = unsafe { super::atof(b"+Inf\0".as_ptr()) };
        assert!(result.is_infinite());
    }

    #[test]
    fn leading_minus_inf() {
        let result = unsafe { super::atof(b"-Inf\0".as_ptr()) };
        assert!(result.is_infinite());
        assert!(result.is_sign_negative());
    }
}
