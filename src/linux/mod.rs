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

use ctypes::{c_char, c_int, c_uint, c_ulong, size_t, ssize_t};
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
pub const O_LARGEFILE: c_int = 0o00100000;

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
pub unsafe fn close(fd: c_int) -> ssize_t {
    syscall!(CLOSE, fd) as ssize_t
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
                   -> ssize_t {
    syscall!(OPENAT, AT_FDCWD, filename, flags, mode) as ssize_t
}

// fs/read_write.c
#[inline(always)]
pub unsafe fn read(fd: c_int, buffer: *mut c_char, count: size_t) -> ssize_t {
    syscall!(READ, fd, buffer, count) as ssize_t
}

// fs/read_write.c
#[inline(always)]
pub unsafe fn write(fd: c_int,
                    buffer: *const c_char,
                    count: size_t)
                    -> ssize_t {
    syscall!(WRITE, fd, buffer, count) as ssize_t
}

// fs/ioctl.c
#[inline(always)]
pub unsafe fn ioctl(fd: c_int, cmd: c_uint, arg: c_ulong) -> ssize_t {
    syscall!(IOCTL, fd, cmd, arg) as ssize_t
}

// fs/ioctl.c
#[inline(always)]
pub unsafe fn fsync(fd: c_int) -> ssize_t {
    syscall!(FSYNC, fd) as ssize_t
}

// fs/ioctl.c
#[inline(always)]
pub unsafe fn fdatasync(fd: c_int) -> ssize_t {
    syscall!(FDATASYNC, fd) as ssize_t
}

// TODO?
#[allow(non_camel_case_types)]
pub type mode_t = umode_t;
