//! Show the output of the random number generator

use core::ffi::c_int;

fn main() {
	let mut distribution = std::collections::BTreeMap::new();
	let mut min = c_int::MAX;
	let mut max = c_int::MIN;

	let mut seedp = 1;
	for i in 0..40 {
		println!("{}/40", i);
		for _ in 0..1_000_000 {
			let random = unsafe { tinyrlibc::rand_r(&mut seedp) };
			if random > max {
				max = random;
			}
			if random < min {
				min = random;
			}
			for place in 0..core::ffi::c_int::BITS {
				let random_bit = (random & (1 << place)) != 0;
				if random_bit {
					distribution
						.entry(place)
						.and_modify(|v| *v += 1)
						.or_insert(1);
				}
			}
		}
	}

	println!("Min value: {min:#10x}");
	println!("Max value: {max:#10x}");
	for (k, v) in distribution.iter() {
		println!("Bit {:02} was set {} times", k, v);
	}
}
