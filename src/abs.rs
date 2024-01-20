//! Rust implementation of C library function `abs`
//!
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::CInt;

/// Rust implementation of C library function `abs`
///
/// ```
/// use tinyrlibc::abs;
/// assert_eq!(abs(-2), 2);
/// ```
#[cfg_attr(feature = "abs", export_name = "abs")]
pub fn abs(i: CInt) -> CInt {
	i.abs()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn neg() {
		assert_eq!(abs(-2), 2);
	}

	#[test]
	fn pos() {
		assert_eq!(abs(3), 3);
	}

	#[test]
	fn zero() {
		assert_eq!(abs(0), 0);
	}
}
