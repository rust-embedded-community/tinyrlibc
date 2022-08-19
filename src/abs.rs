//! Rust implementation of C library function `abs`
//!
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::CInt;

/// Calculates the integer absolute value
///
/// ```
/// use tinyrlibc::abs;
/// assert_eq!(unsafe { abs(-2) }, 2);
/// ```
#[no_mangle]
pub unsafe extern "C" fn abs(i: CInt) -> CInt {
    i.abs()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn neg() {
        assert_eq!(
            unsafe { abs(-2) },
            2
        );
    }

    #[test]
    fn pos() {
        assert_eq!(
            unsafe { abs(3) },
            3
        );
    }

    #[test]
    fn zero() {
        assert_eq!(
            unsafe { abs(0) },
            0
        );
    }
}
