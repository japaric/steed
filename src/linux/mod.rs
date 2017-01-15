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
pub use self::types::{clockid_t, time_t, timespec};

// include/uapi/linux/fcntl.h
pub const AT_FDCWD: c_int = -100;

// include/uapi/asm-generic/fcntl.h
pub const O_ACCMODE: c_int = 0o00000003;
pub const O_RDONLY: c_int = 0o00000000;
pub const O_RDWR: c_int = 0o00000002;
pub const O_WRONLY: c_int = 0o00000001;

// include/uapi/linux/time.h
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_REALTIME: clockid_t = 0;

// kernel/time/posix-timers.c
#[inline(always)]
pub unsafe fn clock_gettime(which_clock: clockid_t,
                            tp: *mut timespec)
                            -> c_int {
    syscall!(CLOCK_GETTIME, which_clock, tp) as isize as c_int
}

// fs/open.c
#[inline(always)]
pub unsafe fn close(fd: c_uint) -> c_int {
    syscall!(CLOSE, fd) as c_int
}

// kernel/exit.c
#[inline(always)]
pub unsafe fn exit(code: c_int) -> ! {
    syscall!(EXIT, code);

    intrinsics::unreachable()
}

// fs/open.c
#[inline(always)]
pub unsafe fn open(filename: *const c_char,
                   flags: c_int,
                   mode: umode_t)
                   -> c_int {
    syscall!(OPENAT, AT_FDCWD, filename, flags, mode) as c_int
}

// fs/read_write.c
#[inline(always)]
pub unsafe fn read(fd: c_uint, buffer: *mut c_char, count: size_t) -> ssize_t {
    syscall!(READ, fd, buffer, count) as ssize_t
}

// fs/read_write.c
#[inline(always)]
pub unsafe fn write(fd: c_uint,
                    buffer: *const c_char,
                    count: size_t)
                    -> ssize_t {
    syscall!(WRITE, fd, buffer, count) as ssize_t
}
