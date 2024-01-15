//! Rust implementation of C library function `memchr`
//!
//! Author: Gyungmin Myung <gmmyung@kaist.ac.kr>
//! Licensed under the MIT license.

use crate::{CInt, CVoid, CSizeT};

#[no_mangle]
pub unsafe extern "C" fn memchr(
    s: *const CVoid,
    c: CInt,
    n: CSizeT,
) -> *mut CVoid {
    core::slice::from_raw_parts(s.cast::<u8>(), n as usize)
        .iter()
        .position(|&x| x == c as u8)
        .map_or(core::ptr::null_mut(), |x| s.add(x) as _)
}

#[cfg(test)]
mod test {
    use crate::{CInt, CSizeT};

    #[test]
    fn find_char() {
        let s = b"hello world\0";
        let result = unsafe { super::memchr(s.as_ptr().cast(), 'w' as CInt, 12 as CSizeT) };
        assert_eq!(result as *const _, s.as_ptr().wrapping_add(6));
    }

    #[test]
    fn find_null() {
        let s = b"hello world\0";
        let result = unsafe { super::memchr(s.as_ptr().cast(), 0, 12 as CSizeT) };
        assert_eq!(result as *const _, s.as_ptr().wrapping_add(11));
    }

    #[test]
    fn find_nothing() {
        let s = b"hello world\0";
        let result = unsafe { super::memchr(s.as_ptr().cast(), 'z' as CInt, 12 as CSizeT) };
        assert_eq!(result, core::ptr::null_mut());
    }
}
