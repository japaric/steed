pub mod ext;
pub mod fd;
pub mod fs;
pub mod io;
pub mod memchr;
// Rust 1.14.0
pub mod os_str;
// Rust 1.14.0
pub mod path;
pub mod pipe;
#[cfg_attr(not(issue = "11"), allow(unused_imports))]
#[cfg_attr(not(issue = "11"), allow(unused_variables))]
pub mod process;
pub mod os;
pub mod rand;
pub mod time;
// Rust 1.14.0
pub mod net;

pub use os::linux as platform;

use io::ErrorKind;
use io::Error;
use io::Result;

// Generated from the Linux source tree using generate/errno.py
pub mod errno;

mod libc {
    pub use ctypes::c_int;
    pub use super::errno::*;
}

// Rust 1.14.0
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

pub fn cvt(ret: isize) -> Result<usize> {
    if ret < 0 {
        assert!(ret >= -0x7fff_ffff);
        Err(Error::from_raw_os_error(-ret as i32))
    } else {
        Ok(ret as usize)
    }
}

pub fn cvt_r<F: FnMut() -> isize>(mut f: F) -> Result<usize> {
    loop {
        match cvt(f()) {
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            other => return other,
        }
    }
}

pub fn cleanup() { }
