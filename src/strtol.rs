//! Rust implementation of C library function `strtol`
//!
//! Original code from the `c-ward` project.
//! Licensed under the MIT license.

use crate::{errno::*, CChar, CInt, CIntMax, CLong, CLongLong, CUIntMax, CULong, CULongLong};

#[cfg(feature = "atoi")]
#[no_mangle]
pub unsafe extern "C" fn atoi(s: *const CChar) -> CInt {
	r_atoi(s)
}

#[cfg(feature = "strtol")]
#[no_mangle]
pub unsafe extern "C" fn strtol(s: *const CChar, endptr: *mut *mut CChar, base: CInt) -> CLong {
	r_strtol(s, endptr, base)
}

#[cfg(feature = "strtoul")]
#[no_mangle]
pub unsafe extern "C" fn strtoul(s: *const CChar, endptr: *mut *mut CChar, base: CInt) -> CULong {
	r_strtoul(s, endptr, base)
}

#[cfg(feature = "strtoll")]
#[no_mangle]
pub unsafe extern "C" fn strtoll(
	s: *const CChar,
	endptr: *mut *mut CChar,
	base: CInt,
) -> CLongLong {
	r_strtoll(s, endptr, base)
}

#[cfg(feature = "strtoull")]
#[no_mangle]
pub unsafe extern "C" fn strtoull(
	s: *const CChar,
	endptr: *mut *mut CChar,
	base: CInt,
) -> CULongLong {
	r_strtoull(s, endptr, base)
}

#[cfg(feature = "strtoimax")]
#[no_mangle]
pub unsafe extern "C" fn strtoimax(
	s: *const CChar,
	endptr: *mut *mut CChar,
	base: CInt,
) -> CIntMax {
	r_strtoimax(s, endptr, base)
}

#[cfg(feature = "strtoumax")]
#[no_mangle]
pub unsafe extern "C" fn strtoumax(
	s: *const CChar,
	endptr: *mut *mut CChar,
	base: CInt,
) -> CUIntMax {
	r_strtoumax(s, endptr, base)
}

pub(crate) unsafe fn r_atoi(s: *const CChar) -> CInt {
	r_strtol(s, core::ptr::null_mut(), 10) as CInt
}

pub(crate) unsafe fn r_strtol(s: *const CChar, endptr: *mut *mut CChar, base: CInt) -> CLong {
	r_strtox(s, endptr, base, CLong::MIN as _, CLong::MAX as _) as CLong
}

pub(crate) unsafe fn r_strtoul(s: *const CChar, endptr: *mut *mut CChar, base: CInt) -> CULong {
	r_strtox(s, endptr, base, 0, CULong::MAX as _) as CULong
}

pub(crate) unsafe fn r_strtoll(s: *const CChar, endptr: *mut *mut CChar, base: CInt) -> CLongLong {
	r_strtox(s, endptr, base, CLongLong::MIN, CLongLong::MAX as _) as CLongLong
}

pub(crate) unsafe fn r_strtoull(
	s: *const CChar,
	endptr: *mut *mut CChar,
	base: CInt,
) -> CULongLong {
	r_strtox(s, endptr, base, 0, CULongLong::MAX) as CULongLong
}

pub(crate) unsafe fn r_strtoimax(s: *const CChar, endptr: *mut *mut CChar, base: CInt) -> CIntMax {
	r_strtox(s, endptr, base, CIntMax::MIN, CIntMax::MAX as _) as CIntMax
}

pub(crate) unsafe fn r_strtoumax(s: *const CChar, endptr: *mut *mut CChar, base: CInt) -> CUIntMax {
	r_strtox(s, endptr, base, 0, CUIntMax::MAX) as CUIntMax
}

pub(crate) unsafe fn r_strtox(
	s: *const CChar,
	endptr: *mut *mut CChar,
	base: CInt,
	min: CIntMax,
	max: CUIntMax,
) -> CUIntMax {
	if !(0..=36).contains(&base) {
		set_errno(errno(EINVAL));
		return 0;
	}

	// Skip leading whitespace.
	let mut s = s;
	while r_isspace(CInt::from(*s)) != 0 {
		s = s.add(1);
	}

	// Parse an optional +/- sign.
	let mut negate = false;
	if *s == b'+' as CChar {
		s = s.add(1);
	} else if *s == b'-' as CChar {
		negate = true;
		s = s.add(1);
	}

	// Parse an optional base prefix.
	let mut base: CUIntMax = base as CUIntMax;
	if base == 0 {
		if *s == b'0' as CChar {
			s = s.add(1);
			if (*s == b'x' as CChar || *s == b'X' as CChar) && (*s.add(1)).is_ascii_hexdigit() {
				s = s.add(1);
				base = 16;
			} else {
				base = 8;
			}
		} else {
			base = 10;
		}
	} else if base == 16
		&& *s == b'0' as CChar
		&& (*s.add(1) == b'x' as CChar || *s.add(1) == b'X' as CChar)
		&& (*s.add(2)).is_ascii_hexdigit()
	{
		s = s.add(2);
	}

	// Parse the digits.
	let mut overflow = false;
	let mut num: CUIntMax = 0;
	loop {
		let digit: CUIntMax = match *s {
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
			if (num as CIntMax) < min / base as CIntMax {
				overflow = true;
			}
		} else if num > max / base {
			overflow = true;
		}
		num = num.wrapping_mul(base);

		if negate && min != 0 {
			if (num as CIntMax) < min + digit as CIntMax {
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
		set_errno(errno(ERANGE));
		return if negate && min != 0 {
			min as CUIntMax
		} else {
			max
		};
	}

	// Perform negation if requested.
	if negate && min == 0 {
		num = num.wrapping_neg();
	}

	// Return a successful result.
	num as CUIntMax
}

#[cfg(feature = "isspace")]
#[no_mangle]
pub unsafe extern "C" fn isspace(c: CInt) -> CInt {
	r_isspace(c) as CInt
}

#[cfg(feature = "isdigit")]
#[no_mangle]
pub unsafe extern "C" fn isdigit(c: CInt) -> CInt {
	r_isdigit(c) as CInt
}

#[cfg(feature = "isalpha")]
#[no_mangle]
pub unsafe extern "C" fn isalpha(c: CInt) -> CInt {
	r_isalpha(c) as CInt
}

#[cfg(feature = "isupper")]
#[no_mangle]
pub unsafe extern "C" fn isupper(c: CInt) -> CInt {
	r_isupper(c) as CInt
}

pub(crate) fn r_isspace(argument: CInt) -> CInt {
	match argument as CChar {
		b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c => 1,
		_ => 0,
	}
}

pub(crate) fn r_isdigit(argument: CInt) -> CInt {
	(argument as CChar).is_ascii_digit() as CInt
}

pub(crate) fn r_isalpha(argument: CInt) -> CInt {
	(argument as CChar).is_ascii_alphabetic() as CInt
}

pub(crate) fn r_isupper(argument: CInt) -> CInt {
	(argument as CChar).is_ascii_uppercase() as CInt
}

#[cfg(test)]
mod tests {
	use core::ptr::null_mut;

	use super::*;

	#[test]
	fn parse_multi_string() {
		let string = b"10 200000000000000000000000000000 30 -40\0";

		let mut s = string.as_ptr() as *mut CChar;
		let results = [
			(10, unsafe { s.offset(2) }),
			(CULong::MAX, unsafe { s.offset(33) }),
			(30, unsafe { s.offset(36) }),
			(-40i32 as CULong, unsafe { s.offset(40) }),
		];

		for (result_number, result_ptr) in results {
			let number = unsafe { r_strtoul(s, &mut s as *mut _, 10) };

			assert_eq!(s, result_ptr);
			assert_eq!(number, result_number);
		}
	}

	#[test]
	fn parse_hex() {
		assert_eq!(
			unsafe { r_strtoul(b"0xAA123\0".as_ptr(), null_mut(), 0) },
			0xAA123
		);
		assert_eq!(
			unsafe { r_strtoul(b"0X00\0".as_ptr(), null_mut(), 0) },
			0x00
		);
		assert_eq!(
			unsafe { r_strtoul(b"-0x123456F\0".as_ptr(), null_mut(), 0) },
			(-0x123456Fi32) as _
		);
	}
}
