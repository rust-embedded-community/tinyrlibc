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

	use crate::{strcmp::strcmp, CChar, CInt, CLong, CLongLong, CUInt, CULong, CULongLong};

	#[test]
	fn plain_string() {
		let mut buf = [b'\0'; 32];
		assert_eq!(
			unsafe { snprintf(buf.as_mut_ptr(), buf.len(), "Hi\0".as_ptr()) },
			2,
			"{}",
			String::from_utf8_lossy(&buf).escape_debug(),
		);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"Hi\0" as *const u8) },
			0
		);
	}

	#[test]
	fn strings() {
		let mut buf = [b'\0'; 32];
		assert_eq!(
			unsafe {
				snprintf(
					buf.as_mut_ptr(),
					buf.len(),
					"%s, %s!\0".as_ptr(),
					"Hello\0".as_ptr(),
					"World\0".as_ptr(),
				)
			},
			13,
			"{}",
			String::from_utf8_lossy(&buf).escape_debug(),
		);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"Hello, World!\0" as *const u8) },
			0
		);
	}

	#[test]
	fn size() {
		let mut buf = [b'\0'; 32];
		assert_eq!(
			unsafe {
				snprintf(
					buf.as_mut_ptr(),
					buf.len(),
					"%zx\0".as_ptr(),
					0x1000_0000usize,
				)
			},
			8
		);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"10000000\0" as *const u8) },
			0
		);
	}

	#[test]
	fn numbers() {
		let mut buf = [b'\0'; 64];
		assert_eq!(
			unsafe {
				snprintf(
					buf.as_mut_ptr(),
					buf.len(),
					"%u %lu %llu %d %ld %lld %x %lx %llX\0".as_ptr(),
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
			53,
			"{}",
			String::from_utf8_lossy(&buf).escape_debug(),
		);
		assert_eq!(
			unsafe {
				strcmp(
					buf.as_ptr() as *const u8,
					b"100 100 100 -100 -100 -100 cafe1234 cafe1234 CAFE1234\0" as *const u8,
				)
			},
			0
		);
	}

	#[test]
	fn non_null_terminated_with_length() {
		let mut buf = [b'\0'; 64];
		assert_eq!(
			unsafe {
				snprintf(
					buf.as_mut_ptr(),
					buf.len(),
					"%.*s\0".as_ptr(),
					5,
					"01234567890123456789\0".as_ptr(),
				)
			},
			5,
			"{}",
			String::from_utf8_lossy(&buf).escape_debug(),
		);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"01234\0" as *const u8,) },
			0
		);
		assert_eq!(
			unsafe {
				snprintf(
					buf.as_mut_ptr(),
					buf.len(),
					"%.10s\0".as_ptr(),
					"01234567890123456789\0".as_ptr(),
				)
			},
			10,
			"{}",
			String::from_utf8_lossy(&buf).escape_debug(),
		);
		assert_eq!(
			unsafe { strcmp(buf.as_ptr() as *const u8, b"0123456789\0" as *const u8,) },
			0
		);
	}

	#[test]
	fn number_with_padding() {
		let mut buf = [b'\0'; 128];
		assert_eq!(
			unsafe {
				snprintf(
					buf.as_mut_ptr(),
					buf.len(),
					"%5u %5lu %5llu %5d %5ld %5lld %10x %10lx %10llX\0".as_ptr(),
					CUInt::from(123u8),
					CULong::from(123u8),
					CULongLong::from(123u8),
					CInt::from(-123i8),
					CLong::from(-123i8),
					CLongLong::from(-123i8),
					CUInt::from(0xcafe1234u32),
					CULong::from(0xcafe1234u32),
					CULongLong::from(0xcafe1234u32),
				)
			},
			68,
			"{}",
			String::from_utf8_lossy(&buf).escape_debug(),
		);
		assert_eq!(
			unsafe {
				strcmp(
					buf.as_ptr() as *const u8,
					b"  123   123   123  -123  -123  -123   cafe1234   cafe1234   CAFE1234\0"
						as *const u8,
				)
			},
			0
		);
	}

	#[test]
	fn number_with_zero_padding() {
		let mut buf = [b'\0'; 128];
		assert_eq!(
			unsafe {
				snprintf(
					buf.as_mut_ptr(),
					buf.len(),
					"%05u %05lu %05llu %05d %05ld %05lld %010x %010lx %010llX\0".as_ptr(),
					CUInt::from(123u8),
					CULong::from(123u8),
					CULongLong::from(123u8),
					CInt::from(-123i8),
					CLong::from(-123i8),
					CLongLong::from(-123i8),
					CUInt::from(0xcafe1234u32),
					CULong::from(0xcafe1234u32),
					CULongLong::from(0xcafe1234u32),
				)
			},
			68,
			"{}",
			String::from_utf8_lossy(&buf).escape_debug(),
		);
		assert_eq!(
			unsafe {
				strcmp(
					buf.as_ptr() as *const u8,
					b"00123 00123 00123 -0123 -0123 -0123 00cafe1234 00cafe1234 00CAFE1234\0"
						as *const u8,
				)
			},
			0
		);
	}

	#[test]
	fn number_with_precision() {
		let mut buf = [b'\0'; 128];
		assert_eq!(
			unsafe {
				snprintf(
					buf.as_mut_ptr(),
					buf.len(),
					"%.5u %.5lu %.5llu %.5d %.5ld %.5lld %.10x %.10lx %.10llX\0".as_ptr(),
					CUInt::from(123u8),
					CULong::from(123u8),
					CULongLong::from(123u8),
					CInt::from(-123i8),
					CLong::from(-123i8),
					CLongLong::from(-123i8),
					CUInt::from(0xcafe1234u32),
					CULong::from(0xcafe1234u32),
					CULongLong::from(0xcafe1234u32),
				)
			},
			71,
			"{}",
			String::from_utf8_lossy(&buf).escape_debug(),
		);
		assert_eq!(
			unsafe {
				strcmp(
					buf.as_ptr() as *const u8,
					b"00123 00123 00123 -00123 -00123 -00123 00cafe1234 00cafe1234 00CAFE1234\0"
						as *const u8,
				)
			},
			0
		);
	}

	#[test]
	fn number_with_width_and_precision() {
		let mut buf = [b'\0'; 128];
		assert_eq!(
			unsafe {
				snprintf(
					buf.as_mut_ptr(),
					buf.len(),
					"%10.5u %2.5lu %10.5llu %10.5d %2.5ld %10.5lld %15.10x %5.10lx %5.3llX\0"
						.as_ptr(),
					CUInt::from(123u8),
					CULong::from(123u8),
					CULongLong::from(123u8),
					CInt::from(-123i8),
					CLong::from(-123i8),
					CLongLong::from(-123i8),
					CUInt::from(0xcafe1234u32),
					CULong::from(0xcafe1234u32),
					CULongLong::from(0xcafe1234u32),
				)
			},
			92,
			"{}",
			String::from_utf8_lossy(&buf).escape_debug(),
		);
		assert_eq!(
			unsafe {
				strcmp(
					buf.as_ptr() as *const u8,
					b"     00123 00123      00123     -00123 -00123     -00123      00cafe1234 00cafe1234 CAFE1234\0"
						as *const u8,
				)
			},
			0
		);
	}
}
