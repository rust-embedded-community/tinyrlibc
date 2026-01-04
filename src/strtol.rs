//! Rust implementation of C library function `strtol`
//!
//! Original code from the `c-ward` project.
//! Licensed under the MIT license.

use core::ffi::{c_char, c_int, c_long, c_longlong, c_ulong, c_ulonglong};

/// Rust implementation of C library function `atoi`
#[cfg_attr(feature = "atoi", no_mangle)]
pub unsafe extern "C" fn atoi(s: *const c_char) -> c_int {
	strtol(s, core::ptr::null_mut(), 10) as c_int
}

/// Rust implementation of C library function `atol`
#[cfg_attr(feature = "strtol", no_mangle)]
pub unsafe extern "C" fn strtol(
	s: *const c_char,
	endptr: *mut *const c_char,
	base: c_int,
) -> c_long {
	strtox(s, endptr, base, c_long::MIN as _, c_long::MAX as _) as c_long
}

/// Rust implementation of C library function `strtoul`
#[cfg_attr(not(feature = "strtoul"), export_name = "tinyrlibc_strtoul")]
#[cfg_attr(feature = "strtoul", no_mangle)]
pub unsafe extern "C" fn strtoul(
	s: *const c_char,
	endptr: *mut *const c_char,
	base: c_int,
) -> c_ulong {
	strtox(s, endptr, base, 0, c_ulong::MAX as _) as c_ulong
}

/// Rust implementation of C library function `strtoll`
#[cfg_attr(feature = "strtoll", no_mangle)]
pub unsafe extern "C" fn strtoll(
	s: *const c_char,
	endptr: *mut *const c_char,
	base: c_int,
) -> c_longlong {
	strtox(s, endptr, base, c_longlong::MIN, c_longlong::MAX as _) as c_longlong
}

/// Rust implementation of C library function `strtoull`
#[cfg_attr(feature = "strtoull", no_mangle)]
pub unsafe extern "C" fn strtoull(
	s: *const c_char,
	endptr: *mut *const c_char,
	base: c_int,
) -> c_ulonglong {
	strtox(s, endptr, base, 0, c_ulonglong::MAX) as c_ulonglong
}

/// Rust implementation of C library function `strtoimax`
#[cfg_attr(feature = "strtoimax", no_mangle)]
pub unsafe extern "C" fn strtoimax(
	s: *const c_char,
	endptr: *mut *const c_char,
	base: c_int,
) -> c_longlong {
	strtox(s, endptr, base, c_longlong::MIN, c_ulonglong::MAX) as c_longlong
}

/// Rust implementation of C library function `strtoumax`
#[cfg_attr(feature = "strtoumax", no_mangle)]
pub unsafe extern "C" fn strtoumax(
	s: *const c_char,
	endptr: *mut *const c_char,
	base: c_int,
) -> c_ulonglong {
	strtox(s, endptr, base, 0, c_ulonglong::MAX) as c_ulonglong
}

pub unsafe fn strtox(
	s: *const c_char,
	endptr: *mut *const c_char,
	base: c_int,
	min: c_longlong,
	max: c_ulonglong,
) -> c_ulonglong {
	if !(0..=36).contains(&base) {
		// TODO: set errno to EINVAL
		return 0;
	}

	// Skip leading whitespace.
	let mut s = s;
	while isspace(c_int::from(*s)) != 0 {
		s = s.add(1);
	}

	// Parse an optional +/- sign.
	let mut negate = false;
	if *s == b'+' as c_char {
		s = s.add(1);
	} else if *s == b'-' as c_char {
		negate = true;
		s = s.add(1);
	}

	const LITTLE_X: c_char = b'x' as c_char;
	const BIG_X: c_char = b'X' as c_char;
	const ZERO: c_char = b'0' as c_char;

	// Parse an optional base prefix.
	let mut base: c_ulonglong = base as c_ulonglong;
	if base == 0 {
		if *s == ZERO {
			s = s.add(1);
			if (*s == LITTLE_X || *s == BIG_X) && (*s.add(1) as u8).is_ascii_hexdigit() {
				s = s.add(1);
				base = 16;
			} else {
				base = 8;
			}
		} else {
			base = 10;
		}
	} else if base == 16
		&& *s == ZERO
		&& (*s.add(1) == LITTLE_X || *s.add(1) == BIG_X)
		&& (*s.add(2) as u8).is_ascii_hexdigit()
	{
		s = s.add(2);
	}

	// Parse the digits.
	let mut overflow = false;
	let mut num: c_ulonglong = 0;
	loop {
		let digit: c_ulonglong = match *s as u8 {
			x @ b'0'..=b'9' => x - b'0',
			x @ b'a'..=b'z' => x - b'a' + 10,
			x @ b'A'..=b'Z' => x - b'A' + 10,
			_ => break,
		}
		.into();
		if digit >= base {
			break;
		}

		if negate && min != 0 {
			if (num as c_longlong) < min / base as c_longlong {
				overflow = true;
			}
		} else if num > max / base {
			overflow = true;
		}
		num = num.wrapping_mul(base);

		if negate && min != 0 {
			if (num as c_longlong) < min + digit as c_longlong {
				overflow = true;
			}
			num = num.wrapping_sub(digit);
		} else {
			if num > max - digit {
				overflow = true;
			}
			num = num.wrapping_add(digit);
		}

		s = s.add(1);
	}

	// If requested, report the end position.
	if !endptr.is_null() {
		*endptr = s.cast_mut();
	}

	// Report overflow.
	if overflow {
		// TODO: set errno to ERANGE
		return if negate && min != 0 {
			min as c_ulonglong
		} else {
			max
		};
	}

	// Perform negation if requested.
	if negate && min == 0 {
		num = num.wrapping_neg();
	}

	// Return a successful result.
	num as c_ulonglong
}

/// Rust implementation of C library function `isspace`
#[cfg_attr(feature = "isspace", no_mangle)]
pub extern "C" fn isspace(argument: c_int) -> c_int {
	match argument as u8 {
		b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c => 1,
		_ => 0,
	}
}

/// Rust implementation of C library function `isdigit`
#[cfg_attr(feature = "isdigit", no_mangle)]
pub extern "C" fn isdigit(argument: c_int) -> c_int {
	(argument as u8).is_ascii_digit() as c_int
}

/// Rust implementation of C library function `isalpha`
#[cfg_attr(feature = "isalpha", no_mangle)]
pub extern "C" fn isalpha(argument: c_int) -> c_int {
	(argument as u8).is_ascii_alphabetic() as c_int
}

/// Rust implementation of C library function `isupper`
#[cfg_attr(feature = "isupper", no_mangle)]
pub extern "C" fn isupper(argument: c_int) -> c_int {
	(argument as u8).is_ascii_uppercase() as c_int
}

#[cfg(test)]
mod tests {
	use core::ptr::null_mut;

	use super::*;

	#[test]
	fn parse_multi_string() {
		let string = c"10 200000000000000000000000000000 30 -40";

		let mut s = string.as_ptr();
		let results = [
			(10, unsafe { s.offset(2) }),
			(c_ulong::MAX, unsafe { s.offset(33) }),
			(30, unsafe { s.offset(36) }),
			(-40i32 as c_ulong, unsafe { s.offset(40) }),
		];

		for (result_number, result_ptr) in results {
			let number = unsafe { strtoul(s, &mut s, 10) };

			assert_eq!(s, result_ptr);
			assert_eq!(number, result_number);
		}
	}

	#[test]
	fn strtol_min() {
		let string = c"-9223372036854775808";
		let value = unsafe { strtol(string.as_ptr(), core::ptr::null_mut(), 10) };
		assert_eq!(value, i64::MIN);
	}

	#[test]
	fn strtol_max() {
		let string = c"9223372036854775807";
		let value = unsafe { strtol(string.as_ptr(), core::ptr::null_mut(), 10) };
		assert_eq!(value, i64::MAX);
	}

	#[test]
	fn strtoul_max() {
		let string = c"18446744073709551616";
		let value = unsafe { strtoul(string.as_ptr(), core::ptr::null_mut(), 10) };
		assert_eq!(value, u64::MAX);
	}

	#[test]
	fn strtoll_min() {
		let string = c"-9223372036854775808";
		let value = unsafe { strtoll(string.as_ptr(), core::ptr::null_mut(), 10) };
		assert_eq!(value, i64::MIN);
	}

	#[test]
	fn strtoll_max() {
		let string = c"9223372036854775807";
		let value = unsafe { strtoll(string.as_ptr(), core::ptr::null_mut(), 10) };
		assert_eq!(value, i64::MAX);
	}

	#[test]
	fn strtoull_max() {
		let string = c"18446744073709551616";
		let value = unsafe { strtoull(string.as_ptr(), core::ptr::null_mut(), 10) };
		assert_eq!(value, u64::MAX);
	}

	#[test]
	fn parse_hex() {
		assert_eq!(
			unsafe { strtoul(c"0xAA123".as_ptr(), null_mut(), 0) },
			0xAA123
		);
		assert_eq!(unsafe { strtoul(c"0X00".as_ptr(), null_mut(), 0) }, 0x00);
		assert_eq!(
			unsafe { strtoul(c"-0x123456F".as_ptr(), null_mut(), 0) },
			(-0x123456Fi32) as _
		);
	}
}
