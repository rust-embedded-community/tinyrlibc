//! Rust implementation of C library function `errno`
//!
//! Author: Gyungmin Myung <gmmyung@kaist.ac.kr>
//! Licensed under the Blue Oak Model Licence 1.0.0

#[allow(non_camel_case_types)]

use crate::{CInt, CVoid};

static mut _impure_ptr: *mut CVoid = core::ptr::null_mut();

pub struct Errno {
    pub errno: CInt,
}

pub fn errno(errno: CInt) -> Errno{
    Errno { errno }
}

pub fn set_errno(errno: Errno) {}

pub const EPERM: CInt = 1;
pub const ENOENT: CInt = 2;
pub const ESRCH: CInt = 3;
pub const EINTR: CInt = 4;
pub const EIO: CInt = 5;
pub const ENXIO: CInt = 6;
pub const E2BIG: CInt = 7;
pub const ENOEXEC: CInt = 8;
pub const EBADF: CInt = 9;
pub const ECHILD: CInt = 10;
pub const EAGAIN: CInt = 11;
pub const ENOMEM: CInt = 12;
pub const EACCES: CInt = 13;
pub const EFAULT: CInt = 14;
pub const ENOTBLK: CInt = 15;
pub const EBUSY: CInt = 16;
pub const EEXIST: CInt = 17;
pub const EXDEV: CInt = 18;
pub const ENODEV: CInt = 19;
pub const ENOTDIR: CInt = 20;
pub const EISDIR: CInt = 21;
pub const EINVAL: CInt = 22;
pub const ENFILE: CInt = 23;
pub const EMFILE: CInt = 24;
pub const ENOTTY: CInt = 25;
pub const ETXTBSY: CInt = 26;
pub const EFBIG: CInt = 27;
pub const ENOSPC: CInt = 28;
pub const ESPIPE: CInt = 29;
pub const EROFS: CInt = 30;
pub const EMLINK: CInt = 31;
pub const EPIPE: CInt = 32;
pub const EDOM: CInt = 33;
pub const ERANGE: CInt = 34;
pub const EDEADLK: CInt = 35;
pub const ENAMETOOLONG: CInt = 36;
pub const ENOLCK: CInt = 37;
pub const ENOSYS: CInt = 38;
pub const ENOTEMPTY: CInt = 39;
pub const ELOOP: CInt = 40;
pub const EWOULDBLOCK: CInt = EAGAIN;
pub const ENOMSG: CInt = 42;
pub const EIDRM: CInt = 43;
pub const ECHRNG: CInt = 44;
pub const EL2NSYNC: CInt = 45;
pub const EL3HLT: CInt = 46;
pub const EL3RST: CInt = 47;
pub const ELNRNG: CInt = 48;
pub const EUNATCH: CInt = 49;
pub const ENOCSI: CInt = 50;
pub const EL2HLT: CInt = 51;
pub const EBADE: CInt = 52;
pub const EBADR: CInt = 53;
pub const EXFULL: CInt = 54;
pub const ENOANO: CInt = 55;
pub const EBADRQC: CInt = 56;
pub const EBADSLT: CInt = 57;
pub const EDEADLOCK: CInt = EDEADLK;
pub const EBFONT: CInt = 59;
pub const ENOSTR: CInt = 60;
pub const ENODATA: CInt = 61;
pub const ETIME: CInt = 62;
pub const ENOSR: CInt = 63;
pub const ENONET: CInt = 64;
pub const ENOPKG: CInt = 65;
pub const EREMOTE: CInt = 66;
pub const ENOLINK: CInt = 67;
pub const EADV: CInt = 68;
pub const ESRMNT: CInt = 69;
pub const ECOMM: CInt = 70;
pub const EPROTO: CInt = 71;
pub const EMULTIHOP: CInt = 72;
pub const EDOTDOT: CInt = 73;
pub const EBADMSG: CInt = 74;
pub const EOVERFLOW: CInt = 75;
pub const ENOTUNIQ: CInt = 76;
pub const EBADFD: CInt = 77;
pub const EREMCHG: CInt = 78;
pub const ELIBACC: CInt = 79;
pub const ELIBBAD: CInt = 80;
pub const ELIBSCN: CInt = 81;
pub const ELIBMAX: CInt = 82;
pub const ELIBEXEC: CInt = 83;
pub const EILSEQ: CInt = 84;
pub const ERESTART: CInt = 85;
pub const ESTRPIPE: CInt = 86;
pub const EUSERS: CInt = 87;
pub const ENOTSOCK: CInt = 88;
pub const EDESTADDRREQ: CInt = 89;
pub const EMSGSIZE: CInt = 90;
pub const EPROTOTYPE: CInt = 91;
pub const ENOPROTOOPT: CInt = 92;
pub const EPROTONOSUPPORT: CInt = 93;
pub const ESOCKTNOSUPPORT: CInt = 94;
pub const EOPNOTSUPP: CInt = 95;
pub const ENOTSUP: CInt = EOPNOTSUPP;
pub const EPFNOSUPPORT: CInt = 96;
pub const EAFNOSUPPORT: CInt = 97;
pub const EADDRINUSE: CInt = 98;
pub const EADDRNOTAVAIL: CInt = 99;
pub const ENETDOWN: CInt = 100;
pub const ENETUNREACH: CInt = 101;
pub const ENETRESET: CInt = 102;
pub const ECONNABORTED: CInt = 103;
pub const ECONNRESET: CInt = 104;
pub const ENOBUFS: CInt = 105;
pub const EISCONN: CInt = 106;
pub const ENOTCONN: CInt = 107;
pub const ESHUTDOWN: CInt = 108;
pub const ETOOMANYREFS: CInt = 109;
pub const ETIMEDOUT: CInt = 110;
pub const ECONNREFUSED: CInt = 111;
pub const EHOSTDOWN: CInt = 112;
pub const EHOSTUNREACH: CInt = 113;
pub const EALREADY: CInt = 114;
pub const EINPROGRESS: CInt = 115;
pub const ESTALE: CInt = 116;
pub const EUCLEAN: CInt = 117;
pub const ENOTNAM: CInt = 118;
pub const ENAVAIL: CInt = 119;
pub const EISNAM: CInt = 120;
pub const EREMOTEIO: CInt = 121;
pub const EDQUOT: CInt = 122;
pub const ENOMEDIUM: CInt = 123;
pub const EMEDIUMTYPE: CInt = 124;
pub const ECANCELED: CInt = 125;
pub const ENOKEY: CInt = 126;
pub const EKEYEXPIRED: CInt = 127;
pub const EKEYREVOKED: CInt = 128;
pub const EKEYREJECTED: CInt = 129;
pub const EOWNERDEAD: CInt = 130;
pub const ENOTRECOVERABLE: CInt = 131;
pub const ERFKILL: CInt = 132;
