
use crate::{CChar, CFloat, CDouble, errno::*, strtod::*};
use core::str::FromStr;
use core::{slice, str};

#[no_mangle]
pub unsafe extern "C" fn strtof(nptr: *const CChar, endptr: *mut *mut CChar) -> CFloat {

    let nptr = nptr.cast::<u8>();
    let orig = nptr;
    let (nptr, format) = scan_float(nptr);
    let s = make_str(orig, nptr);

    match format {
        Format::Hexadecimal(_any_nonzero) => {
            todo!("hexadecimal float parsing")
        }
        Format::Decimal(any_nonzero) => {
            if let Ok(f) = f32::from_str(s) {
                set_endptr(endptr, nptr);
                set_errno_f32(f, any_nonzero);
                return f;
            }
        }
        Format::Infinity => {
            set_endptr(endptr, nptr);
            return if s.starts_with('-') {
                -f32::INFINITY
            } else {
                f32::INFINITY
            };
        }
        Format::NaN(payload) => {
            let result = if s.starts_with('-') {
                libm::copysignf(f32::NAN, -1.0)
            } else {
                libm::copysignf(f32::NAN, 1.0)
            };
            if let Some(payload) = payload {
                if let Ok(payload) = u32::try_from(payload) {
                    if (libm::copysignf(result, -1.0).to_bits() & payload) == 0 {
                        set_endptr(endptr, nptr);
                        return f32::from_bits(result.to_bits() | payload);
                    }
                }
            } else {
                set_endptr(endptr, nptr);
                return result;
            }
        }
    }

    set_endptr(endptr, orig);
    0.0
}
