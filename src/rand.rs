//! Rust implementation of C library functions `rand` and `srand`
//!
//! Licensed under the Blue Oak Model Licence 1.0.0
use core::{
	ffi::{c_int, c_uint},
	sync::atomic::Ordering,
};

use portable_atomic::AtomicU32;

// static mut RAND: Option<GnuRand> = None;
static RAND_STATE: AtomicU32 = AtomicU32::new(0x0);

/// Rust implementation of C library function `srand`
#[cfg_attr(feature = "rand", no_mangle)]
pub extern "C" fn srand(seed: c_uint) {
	RAND_STATE.store(seed, Ordering::Release);
}

/// Rust implementation of C library function `rand`.
///
/// Returns a pseudo-random integer in the range 0 to [`RAND_MAX`](crate::RAND_MAX) (inclusive).
/// This requires CAS operations. If your platform does not support them natively,
/// you either have to enable the `rand-cs` feature of `tinyrlibc`,
/// or the [`critical-section`](https://docs.rs/portable-atomic/1.9.0/portable_atomic/#optional-features-critical-section) feature,
/// or the [`unsafe-assume-single-core`](https://docs.rs/portable-atomic/1.9.0/portable_atomic/#optional-features-unsafe-assume-single-core) feature
/// in [`portable-atomic`](https://crates.io/crates/portable-atomic).
#[cfg_attr(feature = "rand", no_mangle)]
pub extern "C" fn rand() -> c_int {
	let mut current_state = RAND_STATE.load(Ordering::Relaxed);
	let mut new_state = current_state;
	let mut result = unsafe { crate::rand_r(&mut new_state as *mut _) };

	loop {
		match RAND_STATE.compare_exchange_weak(
			current_state,
			new_state,
			Ordering::SeqCst,
			Ordering::Relaxed,
		) {
			Ok(_) => break,
			Err(c) => current_state = c,
		}
		new_state = current_state;
		result = unsafe { crate::rand_r(&mut new_state as *mut _) };
	}

	result as _
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_rand() {
		assert_eq!(rand(), 1012483);
		assert_eq!(rand(), 1716955678);
		assert_eq!(rand(), 1792309081);
		srand(5);
		assert_eq!(rand(), 234104183);
		assert_eq!(rand(), 1214203243);
		assert_eq!(rand(), 1803669307);
	}
}
