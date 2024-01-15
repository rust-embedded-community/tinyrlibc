//! Rust implementation of C library function `isspace`
//! 
//! Author: Gyungmin Myung <gmmyung@kaist.ac.kr>
//! Licensed under the MIT license.

use crate::{CInt, CChar};
#[no_mangle]
pub extern "C" fn isspace(c: CInt) -> CInt {
    CInt::from(
        c == CInt::from(b' ')
            || c == CInt::from(b'\t')
            || c == CInt::from(b'\n')
            || c == CInt::from(b'\r')
            || c == 0x0b
            || c == 0x0c,
    )
}

#[cfg(test)]
mod test {
    #[test]
    fn space() {
        let result = super::isspace(' ' as i32);
        assert_eq!(result, 1);
    }

    #[test]
    fn tab() {
        let result = super::isspace('\t' as i32);
        assert_eq!(result, 1);
    }

    #[test]
    fn newline() {
        let result = super::isspace('\n' as i32);
        assert_eq!(result, 1);
    }

    #[test]
    fn carriage_return() {
        let result = super::isspace('\r' as i32);
        assert_eq!(result, 1);
    }

    #[test]
    fn form_feed() {
        let result = super::isspace('\x0c' as i32);
        assert_eq!(result, 1);
    }

    #[test]
    fn vertical_tab() {
        let result = super::isspace('\x0b' as i32);
        assert_eq!(result, 1);
    }

    #[test]
    fn zero() {
        let result = super::isspace('0' as i32);
        assert_eq!(result, 0);
    }

    #[test]
    fn a() {
        let result = super::isspace('a' as i32);
        assert_eq!(result, 0);
    }
}
