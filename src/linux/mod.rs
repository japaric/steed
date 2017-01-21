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

pub use self::arch::*;
pub use self::types::*;

// include/uapi/linux/fcntl.h
pub const AT_FDCWD: c_int = -100;
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x100;

// include/uapi/asm-generic/fcntl.h
pub const O_ACCMODE: c_int = 0o00000003;
pub const O_RDONLY: c_int = 0o00000000;
pub const O_RDWR: c_int = 0o00000002;
pub const O_WRONLY: c_int = 0o00000001;
pub const O_LARGEFILE: c_int = 0o00100000;

// include/uapi/linux/stat.h
pub const S_IFREG: c_uint = 0o0100000;
pub const S_IFLNK: c_uint = 0o0120000;
pub const S_IFDIR: c_uint = 0o0040000;
pub const S_IFMT: c_uint = 0o00170000;

// include/uapi/linux/time.h
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_REALTIME: clockid_t = 0;

// include/uapi/asm-generic/fcntl.h
pub const F_GETFL: c_uint = 3;

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

#[cfg(all(target_endian = "big", target_pointer_width = "32"))]
fn high(v: loff_t) -> i32 { (v >> 32) as i32 }
#[cfg(all(target_endian = "big", target_pointer_width = "32"))]
fn low(v: loff_t) -> i32 { (v & 0xffff_ffff) as i32 }

#[cfg(all(target_endian = "little", target_pointer_width = "32"))]
fn high(v: loff_t) -> i32 { (v & 0xffff_ffff) as i32 }
#[cfg(all(target_endian = "little", target_pointer_width = "32"))]
fn low(v: loff_t) -> i32 { (v >> 32) as i32 }

// fs/read_write.c
#[inline(always)]
pub unsafe fn pread64(fd: c_int,
                      buffer: *mut c_char,
                      count: size_t,
                      pos: loff_t)
                      -> ssize_t {
    #[cfg(all(target_pointer_width = "32", not(target_arch = "x86")))]
    #[inline(always)]
    unsafe fn pread64(fd: c_int,
                      buffer: *const c_char,
                      count: size_t,
                      pos: loff_t)
                      -> ssize_t {
        syscall!(PREAD64, fd, buffer, count, 0, high(pos), low(pos)) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn pread64(fd: c_int,
                      buffer: *const c_char,
                      count: size_t,
                      pos: loff_t)
                      -> ssize_t {
        syscall!(PREAD64, fd, buffer, count, high(pos), low(pos)) as ssize_t
    }
    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    unsafe fn pread64(fd: c_int,
                      buffer: *const c_char,
                      count: size_t,
                      pos: loff_t)
                      -> ssize_t {
        syscall!(PREAD64, fd, buffer, count, pos) as ssize_t
    }
    pread64(fd, buffer, count, pos)
}

// fs/read_write.c
#[inline(always)]
pub unsafe fn pwrite64(fd: c_int,
                       buffer: *const c_char,
                       count: size_t,
                       pos: loff_t)
                       -> ssize_t {
    #[cfg(all(target_pointer_width = "32", not(target_arch = "x86")))]
    #[inline(always)]
    unsafe fn pwrite64(fd: c_int,
                       buffer: *const c_char,
                       count: size_t,
                       pos: loff_t)
                       -> ssize_t {
        syscall!(PWRITE64, fd, buffer, count, 0, high(pos), low(pos)) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn pwrite64(fd: c_int,
                       buffer: *const c_char,
                       count: size_t,
                       pos: loff_t)
                       -> ssize_t {
        syscall!(PWRITE64, fd, buffer, count, high(pos), low(pos)) as ssize_t
    }
    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    unsafe fn pwrite64(fd: c_int,
                       buffer: *const c_char,
                       count: size_t,
                       pos: loff_t)
                       -> ssize_t {
        syscall!(PWRITE64, fd, buffer, count, pos) as ssize_t
    }
    pwrite64(fd, buffer, count, pos)
}

// fs/open.c
#[inline(always)]
pub unsafe fn ftruncate64(fd: c_int, length: loff_t) -> ssize_t {
    #[cfg(all(target_pointer_width = "32", not(target_arch = "x86")))]
    #[inline(always)]
    unsafe fn ftruncate64(fd: c_int, length: loff_t) -> ssize_t {
        syscall!(FTRUNCATE64, fd, 0, high(length), low(length)) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn ftruncate64(fd: c_int, length: loff_t) -> ssize_t {
        syscall!(FTRUNCATE64, fd, high(length), low(length)) as ssize_t
    }
    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    unsafe fn ftruncate64(fd: c_int, length: loff_t) -> ssize_t {
        syscall!(FTRUNCATE, fd, length) as ssize_t
    }
    ftruncate64(fd, length)
}

// fs/ioctl.c
#[inline(always)]
pub unsafe fn ioctl(fd: c_int, cmd: c_uint, arg: c_ulong) -> ssize_t {
    syscall!(IOCTL, fd, cmd, arg) as ssize_t
}

// fs/sync.c
#[inline(always)]
pub unsafe fn fsync(fd: c_int) -> ssize_t {
    syscall!(FSYNC, fd) as ssize_t
}

// fs/sync.c
#[inline(always)]
pub unsafe fn fdatasync(fd: c_int) -> ssize_t {
    syscall!(FDATASYNC, fd) as ssize_t
}

// fs/stat.c
#[inline(always)]
pub unsafe fn fstat64(fd: c_int, statbuf: *mut stat64) -> ssize_t {
    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    unsafe fn fstat64(fd: c_int, statbuf: *mut stat64) -> ssize_t {
        syscall!(FSTAT64, fd, statbuf) as ssize_t
    }
    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    unsafe fn fstat64(fd: c_int, statbuf: *mut stat64) -> ssize_t {
        syscall!(FSTAT, fd, statbuf) as ssize_t
    }
    fstat64(fd, statbuf)
}

// fs/stat.c
#[inline(always)]
pub unsafe fn stat64(filename: *const c_char, statbuf: *mut stat64) -> ssize_t {
    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    unsafe fn stat64(filename: *const c_char, statbuf: *mut stat64) -> ssize_t {
        syscall!(FSTATAT64, AT_FDCWD, filename, statbuf, 0) as ssize_t
    }
    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    unsafe fn stat64(filename: *const c_char, statbuf: *mut stat64) -> ssize_t {
        syscall!(NEWFSTATAT, AT_FDCWD, filename, statbuf, 0) as ssize_t
    }
    stat64(filename, statbuf)
}

// fs/stat.c
#[inline(always)]
pub unsafe fn lstat64(filename: *const c_char, statbuf: *mut stat64) -> ssize_t {
    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    unsafe fn lstat64(filename: *const c_char, statbuf: *mut stat64) -> ssize_t {
        syscall!(FSTATAT64, AT_FDCWD, filename, statbuf, AT_SYMLINK_NOFOLLOW) as ssize_t
    }
    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    unsafe fn lstat64(filename: *const c_char, statbuf: *mut stat64) -> ssize_t {
        syscall!(NEWFSTATAT, AT_FDCWD, filename, statbuf, AT_SYMLINK_NOFOLLOW) as ssize_t
    }
    lstat64(filename, statbuf)
}

// fs/stat.c
#[inline(always)]
pub unsafe fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: c_int)
    -> ssize_t
{
    syscall!(READLINKAT, AT_FDCWD, path, buf, bufsiz) as ssize_t
}

// fs/fcntl.c
#[inline(always)]
pub unsafe fn fcntl(fd: c_int, cmd: c_uint, arg: c_ulong) -> ssize_t {
    syscall!(FCNTL, fd, cmd, arg) as ssize_t
}
