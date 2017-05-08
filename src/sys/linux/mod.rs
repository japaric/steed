#![allow(non_camel_case_types)]

// Rust 1.16.0
pub mod args;
// Rust 1.16.0
pub mod env;
pub mod ext;
// Rust 1.16.0 (very close)
pub mod fd;
// Rust 1.16.0 (own implementation of readdir and canonicalize)
pub mod fs;
pub mod memchr;
pub mod mutex;
// Rust 1.16.0 (without error support for `lookup_host`, minor changes)
pub mod net;
// Rust 1.16.0
pub mod os_str;
// parking_lot 0.4.1, parking_lot_core 0.2.1
// https://github.com/Amanieu/parking_lot
// 73542a955a00e1d785d263d01c61c37da5032474
// (adapted)
pub mod parking_lot;
// Rust 1.16.0
pub mod path;
pub mod pipe;
#[cfg_attr(not(issue = "11"), allow(unused_imports))]
#[cfg_attr(not(issue = "11"), allow(unused_variables))]
pub mod process;
pub mod os;
pub mod rand;
pub mod stack_overflow;
pub mod time;
// Rust 1.15.0
pub mod thread;

pub use os::linux as platform;

use intrinsics;
use io::Error;
use io::ErrorKind;
use io::Result;
use libc;

// Rust 1.16.0: src/libstd/sys/unix/mod.rs
pub fn decode_error_kind(errno: i32) -> ErrorKind {
    match errno as libc::c_int {
        libc::ECONNREFUSED => ErrorKind::ConnectionRefused,
        libc::ECONNRESET => ErrorKind::ConnectionReset,
        libc::EPERM | libc::EACCES => ErrorKind::PermissionDenied,
        libc::EPIPE => ErrorKind::BrokenPipe,
        libc::ENOTCONN => ErrorKind::NotConnected,
        libc::ECONNABORTED => ErrorKind::ConnectionAborted,
        libc::EADDRNOTAVAIL => ErrorKind::AddrNotAvailable,
        libc::EADDRINUSE => ErrorKind::AddrInUse,
        libc::ENOENT => ErrorKind::NotFound,
        libc::EINTR => ErrorKind::Interrupted,
        libc::EINVAL => ErrorKind::InvalidInput,
        libc::ETIMEDOUT => ErrorKind::TimedOut,
        libc::EEXIST => ErrorKind::AlreadyExists,

        // These two constants can have the same value on some systems,
        // but different values on others, so we can't use a match
        // clause
        x if x == libc::EAGAIN || x == libc::EWOULDBLOCK =>
            ErrorKind::WouldBlock,

        _ => ErrorKind::Other,
    }
}

pub trait Cvt: Copy {
    fn cvt(self) -> Result<Self>;
}

macro_rules! impl_cvt {
    ($($t:ident)*) => ($(impl Cvt for $t {
        fn cvt(self) -> Result<$t> {
            if self < 0 {
                assert!(self >= -0x7fff_ffff);
                Err(Error::from_raw_os_error(-(self as i32)))
            } else {
                Ok(self)
            }
        }
    })*)
}

impl_cvt! { i32 isize i64 }

pub fn cvt<I: Cvt>(ret: I) -> Result<I> {
    ret.cvt()
}

pub fn cvtu(ret: isize) -> Result<usize> {
    cvt(ret).map(|r| r as usize)
}

pub fn cvt_r<I: Cvt, F: FnMut() -> I>(mut f: F) -> Result<I> {
    loop {
        match cvt(f()) {
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            other => return other,
        }
    }
}

pub unsafe fn abort_internal() -> ! {
    intrinsics::abort()
}
