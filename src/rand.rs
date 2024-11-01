//! Rust implementation of C library functions `rand` and `srand`
//!
//! Licensed under the Blue Oak Model Licence 1.0.0
use core::{
	ffi::{c_int, c_uint},
	sync::atomic::Ordering,
};

use portable_atomic::AtomicU32;

static RAND_STATE: AtomicU32 = AtomicU32::new(1);

/// Rust implementation of C library function `srand`
#[cfg_attr(feature = "rand", no_mangle)]
pub extern "C" fn srand(seed: c_uint) {
	RAND_STATE.store(seed, Ordering::Relaxed);
}

/// Rust implementation of C library function `rand`.
#[cfg_attr(feature = "rand", no_mangle)]
pub extern "C" fn rand() -> c_int {
	let mut state = RAND_STATE.load(Ordering::Relaxed);
	let result = unsafe { crate::rand_r(&mut state) };
	RAND_STATE.store(state);
	result
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_rand() {
		assert_eq!(rand(), 1012484);
		assert_eq!(rand(), 1716955679);
		assert_eq!(rand(), 1792309082);
		srand(5);
		assert_eq!(rand(), 234104184);
		assert_eq!(rand(), 1214203244);
		assert_eq!(rand(), 1803669308);
	}
}
