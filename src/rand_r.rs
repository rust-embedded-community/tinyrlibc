//! Rust implementation of C library function `rand_r`
//!
//! Licensed under the Blue Oak Model Licence 1.0.0
use core::ffi::{c_int, c_uint};

#[cfg_attr(not(feature = "rand_r"), export_name = "tinyrlibc_RAND_MAX")]
#[cfg_attr(feature = "rand_r", no_mangle)]
pub static RAND_MAX: c_int = c_int::MAX;

/// Rust implementation of C library function `rand_r`
///
/// Passing NULL (core::ptr::null()) gives undefined behaviour.
#[cfg_attr(not(feature = "rand_r"), export_name = "tinyrlibc_rand_r")]
#[cfg_attr(feature = "rand_r", no_mangle)]
pub unsafe extern "C" fn rand_r(seedp: *mut c_uint) -> c_int {
	let mut result;

	fn pump(input: c_uint) -> c_uint {
		// This algorithm is mentioned in the ISO C standard
		input.wrapping_mul(1103515245).wrapping_add(12345)
	}

	fn select_top(state: c_uint, bits: usize) -> c_uint {
		// ignore the lower 16 bits, as they are low quality
		(state >> 16) & ((1 << bits) - 1)
	}

	let mut next = *seedp;
	if c_int::MAX == 32767 {
		// pull 15 bits in one go
		next = pump(next);
		result = select_top(next, 15);
	} else {
		// pull 31 bits in three goes
		next = pump(next);
		result = select_top(next, 11) << 20;
		next = pump(next);
		result |= select_top(next, 10) << 10;
		next = pump(next);
		result |= select_top(next, 10);
	}
	*seedp = next;

	result as c_int
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_rand_r() {
		unsafe {
			let mut seed = 5;
			assert_eq!(rand_r(&mut seed), 234104184);
			assert_eq!(rand_r(&mut seed), 1214203244);
			assert_eq!(rand_r(&mut seed), 1803669308);
		}
	}
}
