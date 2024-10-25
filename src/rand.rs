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

/// Rust implementation of C library function `rand`
///
/// Returns a pseudo-random integer in the range 0 to `RAND_MAX` (inclusive).
/// May produce the same value in a row if called from multiple threads on platforms not supporting CAS operations.
#[cfg_attr(feature = "rand", no_mangle)]
pub extern "C" fn rand() -> c_int {
	// Atomically update the global LFSR state using compare_and_swap if available
	#[allow(dead_code)]
	fn with_cas() -> c_int {
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
	// Fallback to non-atomic operation if compare_and_swap is not available
	#[allow(dead_code)]
	fn without_cas() -> c_int {
		let mut current_state = RAND_STATE.load(Ordering::Acquire);
		let result = unsafe { crate::rand_r(&mut current_state as *mut _) };
		RAND_STATE.store(current_state, Ordering::Release);
		result as _
	}
	portable_atomic::cfg_has_atomic_cas! { with_cas() }
	portable_atomic::cfg_no_atomic_cas! { without_cas() }
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
