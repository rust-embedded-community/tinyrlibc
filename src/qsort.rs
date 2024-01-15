//! Rust implementation of C library function `qsort`
//!
//! Original code from the `c-ward` project.
//! Licensed under the MIT license.

use crate::{CVoid, CInt, CSizeT};

#[no_mangle]
pub unsafe extern "C" fn qsort(
    base: *mut CVoid,
    nmemb: CSizeT,
    width: CSizeT,
    compar: Option<unsafe extern "C" fn(*const CVoid, *const CVoid) -> CInt>,
) {

    let compar = compar.unwrap();

    if nmemb <= 1 {
        return;
    }

    let base = base.cast::<u8>();
    let mut gap = nmemb;

    loop {
        gap = next_gap(gap);

        let mut any_swapped = false;
        let mut a = base;
        let mut b = base.add(gap * width);
        for _ in 0..nmemb - gap {
            if compar(a.cast(), b.cast()) > 0 {
                swap(a, b, width);
                any_swapped = true;
            }
            a = a.add(width);
            b = b.add(width);
        }

        if gap <= 1 && !any_swapped {
            break;
        }
    }
}

fn next_gap(gap: CSizeT) -> CSizeT {
    let gap = (gap * 10) / 13;

    if gap == 9 || gap == 10 {
        11 // apply the "rule of 11"
    } else if gap <= 1 {
        1
    } else {
        gap
    }
}

unsafe fn swap(a: *mut u8, b: *mut u8, width: CSizeT) {
    for i in 0..width {
        core::ptr::swap(a.add(i), b.add(i));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_qsort() {
        let mut arr = [5, 4, 3, 2, 1];
        unsafe {
            qsort(
                arr.as_mut_ptr().cast(),
                arr.len(),
                core::mem::size_of::<i32>(),
                Some(cmp),
            );
        }
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    unsafe extern "C" fn cmp(a: *const CVoid, b: *const CVoid) -> CInt {
        let a = a.cast::<i32>();
        let b = b.cast::<i32>();
        if *a < *b {
            -1
        } else if *a > *b {
            1
        } else {
            0
        }
    }
}
