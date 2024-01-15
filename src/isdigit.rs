//! Rust implementation of C library function `isspace`
//!
//! Author: Gyungmin Myung <gmmyung@kaist.ac.kr>
//! Licensed under the MIT license.

use crate::CInt;

#[no_mangle]
pub extern "C" fn isdigit(c: CInt) -> CInt {
    CInt::from(c >= CInt::from(b'0') && c <= CInt::from(b'9'))
}

#[cfg(test)]
mod test {
    use crate::CInt;
	#[test]
	fn all_digits() {
		for i in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] {
			let result = super::isdigit(i as CInt);
			assert_eq!(result, 1);
		}
	}

	#[test]
	fn non_digits() {
		for i in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'] {
			let result = super::isdigit(i as CInt);
			assert_eq!(result, 0);
		}
	}
}
