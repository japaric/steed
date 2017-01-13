#[cfg(target_arch = "aarch64")]
#[path = "aarch64.rs"]
mod arch;

#[cfg(target_arch = "arm")]
#[path = "arm.rs"]
mod arch;

#[cfg(target_arch = "mips")]
#[path = "mips.rs"]
mod arch;

#[cfg(target_arch = "mips64")]
#[path = "mips64.rs"]
mod arch;

#[cfg(target_arch = "powerpc")]
#[path = "powerpc.rs"]
mod arch;

#[cfg(target_arch = "powerpc64")]
#[path = "powerpc64.rs"]
mod arch;

#[cfg(target_arch = "sparc64")]
#[path = "sparc64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

pub mod types;

use core::intrinsics;

use ctypes::{c_char, c_int, c_uint, size_t, ssize_t};
use self::types::umode_t;

pub use self::arch::*;

pub const AT_FDCWD: c_int = -100;
pub const O_ACCMODE: c_int = 0o00000003;
pub const O_RDONLY: c_int = 0o00000000;
pub const O_RDWR: c_int = 0o00000002;
pub const O_WRONLY: c_int = 0o00000001;

#[inline(always)]
pub unsafe fn close(fd: c_uint) -> c_int {
    syscall!(CLOSE, fd) as c_int
}

#[inline(always)]
pub unsafe fn exit(code: c_int) -> ! {
    syscall!(EXIT, code);

    intrinsics::unreachable()
}

#[inline(always)]
pub unsafe fn open(filename: *const c_char,
                   flags: c_int,
                   mode: umode_t)
                   -> c_int {
    syscall!(OPENAT, AT_FDCWD, filename, flags, mode) as c_int
}

#[inline(always)]
pub unsafe fn read(fd: c_uint, buffer: *mut c_char, count: size_t) -> ssize_t {
    syscall!(READ, fd, buffer, count) as ssize_t
}

#[inline(always)]
pub unsafe fn write(fd: c_uint,
                    buffer: *const c_char,
                    count: size_t)
                    -> ssize_t {
    syscall!(WRITE, fd, buffer, count) as ssize_t
}
