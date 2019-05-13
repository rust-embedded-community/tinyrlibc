//! Rust implementation of C library function `strncmp`
//!
//! Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0

/// Rust implementation of C library function `strcmp`
#[no_mangle]
pub unsafe extern "C" fn strcmp(s1: *const crate::CChar, s2: *const crate::CChar) -> crate::CInt {
    for i in 0.. {
        let s1_i = s1.offset(i);
        let s2_i = s2.offset(i);

        let val = *s1_i as i32 - *s2_i as i32;
        if val != 0 || *s1_i == 0 {
            return i32::from(val);
        }
    }
    0
}
