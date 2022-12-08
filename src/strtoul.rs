//! Copyright (c) 1990 Regents of the University of California.
//! All rights reserved.
//!
//! Redistribution and use in source and binary forms, with or without
//! modification, are permitted provided that the following conditions
//! are met:
//! 1. Redistributions of source code must retain the above copyright
//!    notice, this list of conditions and the following disclaimer.
//! 2. Redistributions in binary form must reproduce the above copyright
//!    notice, this list of conditions and the following disclaimer in the
//!    documentation and/or other materials provided with the distribution.
//! 3. [rescinded 22 July 1999]
//! 4. Neither the name of the University nor the names of its contributors
//!    may be used to endorse or promote products derived from this software
//!    without specific prior written permission.
//!
//! THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
//! ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
//! IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
//! ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
//! FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
//! DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
//! OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
//! HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
//! LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
//! OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
//! SUCH DAMAGE.
//!
//! Translated from https://github.com/gcc-mirror/gcc/blob/97d1ed67fc6a5773c8c00875bfa3616a457cf5f9/libiberty/strtoul.c

use crate::{CChar, CInt, CLong, CULong};

/// Rust implementation of C library function [`strtoul`](https://cplusplus.com/reference/cstdlib/strtoul/).
///
/// Passing NULL (core::ptr::null()) gives undefined behaviour.
///
/// Convert a string to an unsigned long integer.
///
/// Ignores `locale' stuff.  Assumes that the upper and lower case
/// alphabets and digits are each contiguous.
#[no_mangle]
pub unsafe extern "C" fn strtoul(
	nptr: *const CChar,
	endptr: *mut *const CChar,
	mut base: CInt,
) -> CULong {
	let mut s = nptr;

	let mut c = *s;
	s = s.offset(1);
	while isspace(c) {
		c = *s;
		s = s.offset(1);
	}

	let neg = if c == b'-' {
		c = *s;
		s = s.offset(1);
		true
	} else {
		if c == b'+' {
			c = *s;
			s = s.offset(1);
		}
		false
	};

	if (base == 0 || base == 16) && c == b'0' && (*s == b'x' || *s == b'X') {
		c = *s.offset(1);
		s = s.offset(2);
		base = 16;
	}

	if base == 0 {
		base = if c == b'0' { 8 } else { 10 };
	}

	let cutoff = CULong::MAX / base as CULong;
	let cutlim = CULong::MAX % base as CULong;

	let mut acc = 0;
	let mut any = 0;

	loop {
		if isdigit(c) {
			c -= b'0';
		} else if isalpha(c) {
			c -= if isupper(c) { b'A' - 10 } else { b'a' - 10 };
		} else {
			break;
		}

		if c as CInt >= base {
			break;
		}

		if any < 0 {
			c = *s;
			s = s.offset(1);
			continue;
		}

		if acc > cutoff || (acc == cutoff && c as CULong > cutlim) {
			any = -1;
			acc = CULong::MAX;
		} else {
			any = 1;
			acc *= base as CULong;
			acc += c as CULong;
		}

		c = *s;
		s = s.offset(1);
	}
	if neg && any > 0 {
		acc = -(acc as CLong) as _;
	}

	if !endptr.is_null() {
		(*endptr) = if any != 0 {
			s.offset(-1)
		} else {
			core::ptr::null()
		};
	}

	acc
}

fn isspace(argument: CChar) -> bool {
	// Rust doesn't support "\v"
	const VERTICAL_TAB: u8 = 0x0B;
	// Rust doesn't support "\f"
	const FEED: u8 = 0x0C;
	const SPACE_CHARACTERS: [u8; 6] = [b' ', b'\n', b'\t', VERTICAL_TAB, FEED, b'\r'];

	SPACE_CHARACTERS.contains(&argument)
}

fn isdigit(argument: CChar) -> bool {
	(b'0'..=b'9').contains(&argument)
}

fn isalpha(argument: CChar) -> bool {
	(b'a'..=b'z').contains(&argument) || (b'A'..=b'Z').contains(&argument)
}

fn isupper(argument: CChar) -> bool {
	(b'A'..=b'Z').contains(&argument)
}

#[cfg(test)]
mod tests {
	use core::ptr::null_mut;

	use super::*;

	#[test]
	fn parse_multi_string() {
		let string = b"10 200000000000000000000000000000 30 -40\0";

		let mut s = string.as_ptr();

		let results = [
			(10, unsafe { s.offset(2) }),
			(CULong::MAX, unsafe { s.offset(33) }),
			(30, unsafe { s.offset(36) }),
			(-40i32 as CULong, unsafe { s.offset(40) }),
		];

		for (result_number, result_ptr) in results {
			let number = unsafe { strtoul(s, &mut s as *mut _, 10) };

			assert_eq!(number, result_number);
			assert_eq!(s, result_ptr);
		}
	}

	#[test]
	fn parse_hex() {
		assert_eq!(
			unsafe { strtoul(b"0xAA123\0".as_ptr(), null_mut(), 0) },
			0xAA123
		);
		assert_eq!(unsafe { strtoul(b"0X00\0".as_ptr(), null_mut(), 0) }, 0x00);
		assert_eq!(
			unsafe { strtoul(b"-0x123456F\0".as_ptr(), null_mut(), 0) },
			(-0x123456Fi32) as _
		);
	}
}
