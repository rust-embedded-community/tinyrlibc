//! Rust implementation of tests for C library function `snprintf`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

#[cfg(feature = "snprintf")]
#[cfg(test)]
mod test {
	extern "C" {
		fn snprintf(buf: *mut CChar, len: usize, fmt: *const CChar, ...) -> i32;
	}

	use core::{ffi::CStr, fmt};
	use std::fmt::format;

	use crate::{strcmp::strcmp, CChar, CInt, CLong, CLongLong, CUInt, CULong, CULongLong};

	/// Make it easier to turn `c"Hello"` into a `*const CChar`
	trait ToByte {
		fn cp(&self) -> *const CChar;
	}

	impl ToByte for &std::ffi::CStr {
		fn cp(&self) -> *const CChar {
			self.as_ptr().cast()
		}
	}

	/// Handle the buffer that `snprintf` needs
	fn asprintf<F>(fmt: &str, expected: &str, f: F)
	where
		F: FnOnce(*mut CChar, usize, *const CChar) -> i32,
	{
		let mut buf = vec![0u8; 128];
		let cfmt = std::ffi::CString::new(fmt).unwrap();
		let res = f(buf.as_mut_ptr(), buf.len(), cfmt.as_ptr().cast());
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
					CUInt::from(100u8),
					CULong::from(100u8),
					CULongLong::from(100u8),
					CInt::from(-100i8),
					CLong::from(-100i8),
					CLongLong::from(-100i8),
					CUInt::from(0xcafe1234u32),
					CULong::from(0xcafe1234u32),
					CULongLong::from(0xcafe1234u32),
				)
			},
		);
	}

	#[test]
	fn int_min() {
		asprintf(
			"%d",
			&format!("{}", CInt::min_value()),
			|buf, len, fmt| unsafe { snprintf(buf, len, fmt, CInt::min_value()) },
		);
		asprintf(
			"%lld",
			&format!("{}", CLongLong::min_value()),
			|buf, len, fmt| unsafe { snprintf(buf, len, fmt, CLongLong::min_value()) },
		);
	}

	#[test]
	fn int_max() {
		asprintf(
			"%d",
			&format!("{}", CInt::max_value()),
			|buf, len, fmt| unsafe { snprintf(buf, len, fmt, CInt::max_value()) },
		);
		asprintf(
			"%lld",
			&format!("{}", CLongLong::max_value()),
			|buf, len, fmt| unsafe { snprintf(buf, len, fmt, CLongLong::max_value()) },
		);
	}

	#[test]
	fn non_null_terminated_with_length() {
		asprintf("%.*s", "01234", |buf, len, fmt: *const u8| unsafe {
			snprintf(buf, len, fmt, 5, c"01234567890123456789".cp())
		});
		asprintf("%.10s", "0123456789", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, c"01234567890123456789".cp())
		});
	}

	#[test]
	fn number_with_padding() {
		asprintf("%5u", "  123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(123u8))
		});
		asprintf("%5lu", "  123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CULong::from(123u8))
		});
		asprintf("%5llu", "  123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CULongLong::from(123u8))
		});
		asprintf("%5d", " -123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CInt::from(-123i8))
		});
		asprintf("%5ld", " -123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CLong::from(-123i8))
		});
		asprintf("%5lld", " -123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CLongLong::from(-123i8))
		});
		asprintf("%10x", "  cafe1234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(0xcafe1234u32))
		});
		asprintf("%10lx", "  cafe1234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CULong::from(0xcafe1234u32))
		});
		asprintf("%10llX", "  CAFE1234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CULongLong::from(0xcafe1234u32))
		});
	}

	#[test]
	fn number_with_zero_padding() {
		asprintf("%05u", "00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(123u8))
		});
		asprintf("%05lu", "00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CULong::from(123u8))
		});
		asprintf("%05llu", "00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CULongLong::from(123u8))
		});
		asprintf("%05d", "-0123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CInt::from(-123i8))
		});
		asprintf("%05ld", "-0123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CLong::from(-123i8))
		});
		asprintf("%05lld", "-0123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CLongLong::from(-123i8))
		});
		asprintf("%010x", "00cafe1234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(0xcafe1234u32))
		});
		asprintf("%010lx", "00cafe1234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CULong::from(0xcafe1234u32))
		});
		asprintf("%010llX", "00CAFE1234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CULongLong::from(0xcafe1234u32))
		});
	}

	#[test]
	fn number_with_precision() {
		asprintf("%.5u", "00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(123u8))
		});
		asprintf("%.5d", "-00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CInt::from(-123i8))
		});
		asprintf("%.10x", "00cafe1234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(0xcafe1234u32))
		});
		asprintf("%.4d", "-0123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CInt::from(-123i8))
		});
		asprintf("%.3d", "-123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CInt::from(-123i8))
		});
		asprintf("%.2u", "123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(123u8))
		});
		asprintf("%.0u", "123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(123u8))
		});
	}

	#[test]
	fn number_with_width_and_precision() {
		asprintf("%10.5u", "     00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(123u8))
		});
		asprintf("%10.5d", "    -00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CInt::from(-123i8))
		});
		asprintf("%15.10x", "     00cafe1234", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(0xcafe1234u32))
		});

		asprintf("%5.5u", "00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(123u8))
		});
		asprintf("%4.5u", "00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(123u8))
		});
		asprintf("%2.5u", "00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(123u8))
		});
		asprintf("%0.5u", "00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(123u8))
		});
		asprintf("%5.4u", " 0123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(123u8))
		});
		asprintf("%5.3u", "  123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(123u8))
		});
		asprintf("%5.0u", "  123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CUInt::from(123u8))
		});

		asprintf("%5.5d", "-00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CInt::from(-123i8))
		});
		asprintf("%4.5d", "-00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CInt::from(-123i8))
		});
		asprintf("%2.5d", "-00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CInt::from(-123i8))
		});
		asprintf("%0.5d", "-00123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CInt::from(-123i8))
		});
		asprintf("%5.4d", "-0123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CInt::from(-123i8))
		});
		asprintf("%5.3d", " -123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CInt::from(-123i8))
		});
		asprintf("%5.0d", " -123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CInt::from(-123i8))
		});

		asprintf("%05.4d", "-0123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CInt::from(-123i8))
		});
		asprintf("%05.3d", " -123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CInt::from(-123i8))
		});
		asprintf("%05.0d", " -123", |buf, len, fmt| unsafe {
			snprintf(buf, len, fmt, CInt::from(-123i8))
		});
	}
}
