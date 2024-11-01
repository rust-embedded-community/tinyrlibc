fn main() {
	let mut distribution = std::collections::BTreeMap::new();

	let mut seedp = 1;
	for _ in 0..4 {
		println!(".");
		for _ in 0..1_000_000 {
			let random = unsafe { tinyrlibc::rand_r(&mut seedp) };
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

	for (k, v) in distribution.iter() {
		println!("{:02} => {}", k, v);
	}
}
