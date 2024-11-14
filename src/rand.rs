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
	// we do this cast to support platforms where c_uint is u16.
	// but it complains on platforms where c_uint is u32.
	#[allow(clippy::unnecessary_cast)]
	RAND_STATE.store(seed as u32, Ordering::Relaxed);
}

/// Rust implementation of C library function `rand`.
#[cfg_attr(feature = "rand", no_mangle)]
pub extern "C" fn rand() -> c_int {
	let mut state = RAND_STATE.load(Ordering::Relaxed) as c_uint;
	let result = unsafe { crate::rand_r(&mut state) };
	RAND_STATE.store(state as u32, Ordering::Relaxed);
	result
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_rand() {
		if c_int::MAX == 32767 || cfg!(feature = "rand_max_i16") {
			srand(1);
			assert_eq!(rand(), 16838);
			assert_eq!(rand(), 5758);
			assert_eq!(rand(), 10113);
			srand(5);
			assert_eq!(rand(), 18655);
			assert_eq!(rand(), 8457);
			assert_eq!(rand(), 10616);
		} else {
			srand(1);
			assert_eq!(rand(), 476707713);
			assert_eq!(rand(), 1186278907);
			assert_eq!(rand(), 505671508);
			srand(5);
			assert_eq!(rand(), 234104184);
			assert_eq!(rand(), 1214203244);
			assert_eq!(rand(), 1803669308);
		}
	}
}
