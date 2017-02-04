pub use ctypes::*;
pub use linux::errno::*;

pub use linux::{gid_t, pid_t, stat64, uid_t};

pub use linux::{O_APPEND, O_CLOEXEC, O_CREAT, O_DIRECTORY, O_EXCL, O_LARGEFILE};
pub use linux::{O_NONBLOCK, O_PATH, O_TRUNC};
pub use linux::{F_DUPFD_CLOEXEC, F_DUPFD, F_GETFL, F_SETFL};
pub use linux::FIOCLEX;
pub use linux::{S_IFMT, S_IFSOCK, S_IFLNK, S_IFREG, S_IFBLK, S_IFDIR, S_IFCHR, S_IFIFO};

#[allow(non_camel_case_types)]
pub type off_t = i64;

use linux;

pub unsafe fn strlen(cs: *const c_char) -> size_t {
    let mut cs = cs;
    let mut count = 0;
    while *cs != 0 {
        cs = cs.offset(1);
        count += 1;
    }
    count
}

#[inline(always)]
pub unsafe fn read(fd: c_int, buffer: *mut c_void, count: size_t) -> ssize_t {
    linux::read(fd, buffer as *mut c_char, count)
}

#[inline(always)]
pub unsafe fn write(fd: c_int, buffer: *const c_void, count: size_t) -> ssize_t {
    linux::write(fd, buffer as *const c_char, count)
}

#[inline(always)]
pub unsafe fn pread64(fd: c_int,
                      buffer: *mut c_void,
                      count: size_t,
                      offset: off_t)
    -> ssize_t
{
    linux::pread64(fd, buffer as *mut c_char, count, offset)
}

#[inline(always)]
pub unsafe fn pwrite64(fd: c_int,
                       buffer: *const c_void,
                       count: size_t,
                       offset: off_t)
    -> ssize_t
{
    linux::pwrite64(fd, buffer as *const c_char, count, offset)
}

#[inline(always)]
pub unsafe fn close(fd: c_int) -> c_int {
    linux::close(fd) as c_int
}

#[inline(always)]
pub unsafe fn fcntl(fd: c_int, cmd: c_uint, arg: c_int) -> c_int {
    linux::fcntl(fd, cmd, arg as c_ulong) as c_int
}

#[inline(always)]
pub unsafe fn ioctl(fd: c_int, cmd: c_uint) -> c_int {
    linux::ioctl(fd, cmd, 0) as c_int
}

// Rust 1.15.0: src/liblibc/src/unix/notbsd/mod.rs
#[allow(non_snake_case)]
pub fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

// Rust 1.15.0: src/liblibc/src/unix/notbsd/mod.rs
#[allow(non_snake_case)]
pub fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

// Rust 1.15.0: src/liblibc/src/unix/notbsd/mod.rs
#[allow(non_snake_case)]
pub fn WEXITSTATUS(status: c_int) -> c_int {
    (status >> 8) & 0xff
}
