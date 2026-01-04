//! Rust implementation of tests for C library function `snprintf`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

#[cfg(test)]
mod test {
	extern "C" {
		fn snprintf(buf: *mut c_char, len: usize, fmt: *const c_char, ...) -> i32;
	}

	use core::{ffi::CStr, fmt};
	use std::fmt::format;

	use crate::strcmp::strcmp;
	use core::ffi::{c_char, c_int, c_long, c_longlong, c_uint, c_ulong, c_ulonglong};

	/// Make it easier to turn `c"Hello"` into a `*const c_char`
	trait ToByte {
		fn cp(&self) -> *const c_char;
	}

	impl ToByte for &std::ffi::CStr {
		fn cp(&self) -> *const c_char {
			self.as_ptr().cast()
		}
	}

	/// Handle the buffer that `snprintf` needs
	#[track_caller]
	fn asprintf<F>(fmt: &str, expected: &str, f: F)
	where
		F: FnOnce(*mut c_char, usize, *const c_char) -> i32,
	{
		let mut buf = vec![0u8; 128];
		let cfmt = std::ffi::CString::new(fmt).unwrap();
		let res = f(buf.as_mut_ptr().cast(), buf.len(), cfmt.as_ptr().cast());
		if res < 0 {
			panic!("closure returned {}", res);
		}
		// res does not include the trailing NUL that snprintf always outputs (if there's space)
		buf.truncate((res + 1) as usize);
		let cs = std::ffi::CString::from_vec_with_nul(buf)
			.expect("failed to make CString from closure output");
		let cs = cs.to_str().expect("was not UTF-8").to_string();
		assert_eq!(
			cs, expected,
			"fmt '{}', expected '{}', got '{}'",
			fmt, expected, cs
		);
	}

	#[test]
	fn plain_string() {
		asprintf("Hi", "Hi", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt)
		});
	}

	#[test]
	fn strings() {
		asprintf("%s, %s!", "Hello, World!", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c"Hello".cp(), c"World".cp())
		});
	}

	#[test]
	fn size() {
		asprintf("%zx", "10000000", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, 0x1000_0000usize)
		});
	}

	#[test]
	fn numbers() {
		asprintf(
			"%u %lu %llu %d %ld %lld %x %lx %llX",
			"100 100 100 -100 -100 -100 cafe1234 cafe1234 CAFE1234",
			|buf, len, fmt| unsafe {
				snprintf(
					buf,
					len,
					fmt,
					c_uint::from(100u8),
					c_ulong::from(100u8),
					c_ulonglong::from(100u8),
					c_int::from(-100i8),
					c_long::from(-100i8),
					c_longlong::from(-100i8),
					c_uint::from(0xcafe1234u32),
					c_ulong::from(0xcafe1234u32),
					c_ulonglong::from(0xcafe1234u32),
				)
			},
		);
	}

	#[test]
	fn int_min() {
		asprintf(
			"%d",
			&format!("{}", c_int::min_value()),
			|buf, len, fmt| unsafe { snprintf(buf, len, fmt, c_int::min_value()) },
		);
		asprintf(
			"%lld",
			&format!("{}", c_longlong::min_value()),
			|buf, len, fmt| unsafe { snprintf(buf, len, fmt, c_longlong::min_value()) },
		);
	}

	#[test]
	fn int_max() {
		asprintf(
			"%d",
			&format!("{}", c_int::max_value()),
			|buf, len, fmt| unsafe { snprintf(buf, len, fmt, c_int::max_value()) },
		);
		asprintf(
			"%lld",
			&format!("{}", c_longlong::max_value()),
			|buf, len, fmt| unsafe { snprintf(buf, len, fmt, c_longlong::max_value()) },
		);
	}

	#[test]
	fn non_null_terminated_with_length() {
		asprintf("%.*s", "01234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, 5, c"01234567890123456789".cp())
		});
		asprintf("%.10s", "0123456789", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c"01234567890123456789".cp())
		});
	}

	#[test]
	fn number_with_padding() {
		asprintf("%5u", "  123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(123u8))
		});
		asprintf("%5lu", "  123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_ulong::from(123u8))
		});
		asprintf("%5llu", "  123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_ulonglong::from(123u8))
		});
		asprintf("%5d", " -123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_int::from(-123i8))
		});
		asprintf("%5ld", " -123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_long::from(-123i8))
		});
		asprintf("%5lld", " -123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_longlong::from(-123i8))
		});
		asprintf("%10x", "  cafe1234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(0xcafe1234u32))
		});
		asprintf("%10lx", "  cafe1234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_ulong::from(0xcafe1234u32))
		});
		asprintf("%10llX", "  CAFE1234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_ulonglong::from(0xcafe1234u32))
		});
	}

	#[test]
	fn number_with_zero_padding() {
		asprintf("%05u", "00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(123u8))
		});
		asprintf("%05lu", "00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_ulong::from(123u8))
		});
		asprintf("%05llu", "00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_ulonglong::from(123u8))
		});
		asprintf("%05d", "-0123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_int::from(-123i8))
		});
		asprintf("%05ld", "-0123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_long::from(-123i8))
		});
		asprintf("%05lld", "-0123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_longlong::from(-123i8))
		});
		asprintf("%010x", "00cafe1234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(0xcafe1234u32))
		});
		asprintf("%010lx", "00cafe1234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_ulong::from(0xcafe1234u32))
		});
		asprintf("%010llX", "00CAFE1234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_ulonglong::from(0xcafe1234u32))
		});
	}

	#[test]
	fn number_with_precision() {
		asprintf("%.5u", "00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(123u8))
		});
		asprintf("%.5d", "-00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_int::from(-123i8))
		});
		asprintf("%.10x", "00cafe1234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(0xcafe1234u32))
		});
		asprintf("%.4d", "-0123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_int::from(-123i8))
		});
		asprintf("%.3d", "-123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_int::from(-123i8))
		});
		asprintf("%.2u", "123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(123u8))
		});
		asprintf("%.0u", "123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(123u8))
		});
	}

	#[test]
	fn number_with_width_and_precision() {
		asprintf("%10.5u", "     00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(123u8))
		});
		asprintf("%10.5d", "    -00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_int::from(-123i8))
		});
		asprintf("%15.10x", "     00cafe1234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(0xcafe1234u32))
		});

		asprintf("%5.5u", "00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(123u8))
		});
		asprintf("%4.5u", "00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(123u8))
		});
		asprintf("%2.5u", "00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(123u8))
		});
		asprintf("%0.5u", "00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(123u8))
		});
		asprintf("%5.4u", " 0123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(123u8))
		});
		asprintf("%5.3u", "  123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(123u8))
		});
		asprintf("%5.0u", "  123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_uint::from(123u8))
		});

		asprintf("%5.5d", "-00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_int::from(-123i8))
		});
		asprintf("%4.5d", "-00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_int::from(-123i8))
		});
		asprintf("%2.5d", "-00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_int::from(-123i8))
		});
		asprintf("%0.5d", "-00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_int::from(-123i8))
		});
		asprintf("%5.4d", "-0123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_int::from(-123i8))
		});
		asprintf("%5.3d", " -123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_int::from(-123i8))
		});
		asprintf("%5.0d", " -123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_int::from(-123i8))
		});

		asprintf("%05.4d", "-0123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_int::from(-123i8))
		});
		asprintf("%05.3d", " -123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_int::from(-123i8))
		});
		asprintf("%05.0d", " -123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c_int::from(-123i8))
		});
	}
}
