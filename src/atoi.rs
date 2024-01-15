//! Rust implementation of C library function `atoi`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::{CChar, CInt, CLong, CLongLong, isspace, isdigit};

use core::ops::{MulAssign, Neg, SubAssign};

#[no_mangle]
pub unsafe extern "C" fn atoi(s: *const CChar) -> CInt {
    _atoi(s)
}

#[no_mangle]
pub unsafe extern "C" fn atol(s: *const CChar) -> CLong {
    _atoi(s)
}

#[no_mangle]
pub unsafe extern "C" fn atoll(s: *const CChar) -> CLongLong {
    _atoi(s)
}

unsafe fn _atoi<T: MulAssign + SubAssign + Neg<Output = T> + From<u8> + Default>(
    mut s: *const CChar,
) -> T {
    let mut negate = false;
    let mut n = T::default();

    // Skip leading whitespace.
    while isspace(*s as CInt) != 0 {
        s = s.add(1);
    }

    // Handle a sign.
    match *s as u8 {
        b'-' => {
            negate = true;
            s = s.add(1);
        }
        b'+' => {
            s = s.add(1);
        }
        _ => {}
    }

    // Handle digits.
    while isdigit(*s as CInt) != 0 {
        n *= T::from(10u8);
        n -= (*s as u8 - b'0').into();
        s = s.add(1);
    }

    if !negate {
        n = -n;
    }

    n
}

#[cfg(test)]
mod test {
    #[test]
    fn positive() {
        let result = unsafe { super::atoi(b"123\0".as_ptr()) };
        assert_eq!(result, 123);
    }

    #[test]
    fn negative() {
        let result = unsafe { super::atoi(b"-123\0".as_ptr()) };
        assert_eq!(result, -123);
    }

    #[test]
    fn zero() {
        let result = unsafe { super::atoi(b"0\0".as_ptr()) };
        assert_eq!(result, 0);
    }

    #[test]
    fn leading_whitespace() {
        let result = unsafe { super::atoi(b" \t\n\r123\0".as_ptr()) };
        assert_eq!(result, 123);
    }

    #[test]
    fn trailing_whitespace() {
        let result = unsafe { super::atoi(b"123 \t\n\r\0".as_ptr()) };
        assert_eq!(result, 123);
    }
}
