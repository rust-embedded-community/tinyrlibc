//! Rust implementation of C library function `strtol`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::CStringIter;

#[no_mangle]
pub unsafe extern "C" fn strtol(s: *const crate::CChar) -> crate::CLong {
    let mut result: crate::CLong = 0;
    for c in CStringIter::new(s) {
        if c == 0 {
            break;
        }
        if c >= b'0' && c <= b'9' {
            result *= 10;
            result += (c - b'0') as i32;
        } else {
            break;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::strtol;

    #[test]
    fn empty() {
        let result = unsafe { strtol(b"\0".as_ptr()) };
        assert_eq!(result, 0);
    }

    #[test]
    fn one() {
        let result = unsafe { strtol(b"1\0".as_ptr()) };
        assert_eq!(result, 1);
    }

    #[test]
    fn hundredish() {
        let result = unsafe { strtol(b"123\0".as_ptr()) };
        assert_eq!(result, 123);
    }
}
