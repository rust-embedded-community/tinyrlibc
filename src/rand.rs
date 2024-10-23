//! Rust implementation of C library functions `rand` and `srand`
//!
//! Licensed under the Blue Oak Model Licence 1.0.0
use core::ffi::{c_int, c_uint};

struct GnuRand {
	r: [u32; 344],
	n: usize,
}

impl GnuRand {
	pub fn new(mut seed: u32) -> GnuRand {
		let mut r = [0u32; 344];

		if seed == 0 {
			// Make sure seed is not 0
			seed = 1;
		}

		r[0] = seed;
		for i in 1..31 {
			// This does:
			// state[i] = (16807 * state[i - 1]) % 2147483647;
			// but avoids overflowing 31 bits.
			let hi = (r[i - 1] / 127773) as i32;
			let lo = (r[i - 1] % 127773) as i32;
			let mut word = 16807 * lo - 2836 * hi;
			if word < 0 {
				word += i32::MAX;
			}
			r[i] = word as u32;
		}
		for i in 31..34 {
			r[i] = r[i - 31];
		}
		for i in 34..344 {
			r[i] = r[i - 31].wrapping_add(r[i - 3]);
		}

		GnuRand { r, n: 0 }
	}

	pub fn next(&mut self) -> i32 {
		let x = self.r[(self.n + 313) % 344].wrapping_add(self.r[(self.n + 341) % 344]);
		self.r[self.n % 344] = x;
		self.n = (self.n + 1) % 344;
		(x >> 1) as i32
	}
}

static mut RAND: Option<GnuRand> = None;

/// Rust implementation of C library function `srand`
///
/// Relies on [`critical-section`](https://docs.rs/critical-section/1.2.0/critical_section/) for thread-safety
#[cfg_attr(feature = "rand", no_mangle)]
pub extern "C" fn srand(seed: c_uint) {
	let rnd = GnuRand::new(seed);
	critical_section::with(|_| unsafe { RAND = Some(rnd) });
}

/// Rust implementation of C library function `rand`
///
/// Relies on [`critical-section`](https://docs.rs/critical-section/1.2.0/critical_section/) for thread-safety
#[cfg_attr(feature = "rand", no_mangle)]
pub extern "C" fn rand() -> c_int {
	critical_section::with(|_| {
		let rnd = unsafe { RAND.get_or_insert_with(|| GnuRand::new(1)) };
		rnd.next()
	})
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_rand() {
		unsafe {
			// Values taken from glibc implementation
			assert_eq!(rand(), 1804289383);
			assert_eq!(rand(), 846930886);
			assert_eq!(rand(), 1681692777);
		}
	}
	#[test]
	fn test_srand() {
		unsafe {
			srand(5);
			// Values taken from glibc implementation
			assert_eq!(rand(), 590011675);
			assert_eq!(rand(), 99788765);
			assert_eq!(rand(), 2131925610);
		}
	}
}
