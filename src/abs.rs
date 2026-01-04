//! Rust implementation of C library function `abs`
//!
//! Licensed under the Blue Oak Model Licence 1.0.0

use core::ffi::c_int;

/// Rust implementation of C library function `abs`
#[cfg_attr(feature = "abs", no_mangle)]
pub extern "C" fn abs(i: c_int) -> c_int {
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
