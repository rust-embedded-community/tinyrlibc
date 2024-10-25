//! Rust implementation of C library function `rand_r`
//!
//! Licensed under the Blue Oak Model Licence 1.0.0
use core::ffi::{c_int, c_uint};

/// Rust implementation of C library function `rand_r`
///
/// Passing NULL (core::ptr::null()) gives undefined behaviour.
#[cfg_attr(not(feature = "rand_r"), export_name = "tinyrlibc_rand_r")]
#[cfg_attr(feature = "rand_r", no_mangle)]
pub unsafe extern "C" fn rand_r(seedp: *mut c_uint) -> c_int {
	// This algorithm is mentioned in the ISO C standard, here extended for 32 bits.
	let mut next = *seedp;
	let mut result: c_int;

	next = next.wrapping_mul(1103515245);
	next = next.wrapping_add(12345);
	result = ((next / 65536) % 2048) as c_int;

	next = next.wrapping_mul(1103515245);
	next = next.wrapping_add(12345);
	result <<= 10;
	result ^= ((next / 65536) % 1024) as c_int;

	next = next.wrapping_mul(1103515245);
	next = next.wrapping_add(12345);
	result <<= 10;
	result ^= ((next / 65536) % 1024) as c_int;

	*seedp = next;

	result
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_rand_r() {
		unsafe {
			let mut seed = 5;
			// Values taken from glibc implementation
			assert_eq!(rand_r(&mut seed), 234104183);
			assert_eq!(rand_r(&mut seed), 1214203243);
			assert_eq!(rand_r(&mut seed), 1803669307);
		}
	}
}
