#![allow(non_camel_case_types)]

use cmp;
use linux;

pub use ctypes::*;
pub use linux::errno::*;

pub use linux::{gid_t, pid_t, stat64, time_t, timespec, uid_t};

pub use linux::{DT_BLK, DT_CHR, DT_DIR, DT_FIFO, DT_LNK, DT_REG, DT_SOCK};
pub use linux::{F_DUPFD_CLOEXEC, F_DUPFD, F_GETFL, F_SETFL};
pub use linux::{FIOCLEX};
pub use linux::{O_ACCMODE, O_APPEND, O_CLOEXEC, O_CREAT, O_DIRECTORY, O_EXCL};
pub use linux::{O_LARGEFILE, O_NONBLOCK, O_PATH, O_RDONLY, O_RDWR, O_TRUNC};
pub use linux::{O_WRONLY};
pub use linux::{S_IFMT, S_IFSOCK, S_IFLNK, S_IFREG, S_IFBLK, S_IFDIR, S_IFCHR};
pub use linux::{S_IFIFO};
pub use linux::{SEEK_CUR, SEEK_END, SEEK_SET};

pub use linux::{close, fdatasync, fstat64, fsync, ftruncate64, link, lstat64};
pub use linux::{pread64, pwrite64, read, rename, rmdir, symlink, unlink, write};

pub type off64_t = i64;
pub type mode_t = u32;

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
pub unsafe fn fcntl(fd: c_int, cmd: c_uint, arg: c_int) -> c_int {
    linux::fcntl(fd, cmd, arg as c_ulong) as c_int
}

#[inline(always)]
pub unsafe fn ioctl(fd: c_int, cmd: c_uint) -> c_int {
    linux::ioctl(fd, cmd, 0) as c_int
}

#[inline(always)]
pub unsafe fn open64(pathname: *const c_char, flags: c_int, mode: c_int) -> c_int {
    linux::open(pathname, flags | O_LARGEFILE, mode as linux::umode_t)
}

#[inline(always)]
pub unsafe fn readlink(pathname: *const c_char,
                       buf: *mut c_char,
                       bufsiz: size_t)
    -> ssize_t
{
    linux::readlink(pathname,
                    buf,
                    cmp::min(bufsiz, <c_int>::max_value() as size_t) as c_int)
}

#[inline(always)]
pub unsafe fn fchmod(fd: c_int, mode: mode_t) -> c_int {
    linux::fchmod(fd, mode as linux::umode_t)
}

#[inline(always)]
pub unsafe fn chmod(filename: *const c_char, mode: mode_t) -> c_int {
    linux::chmod(filename, mode as linux::umode_t)
}

#[inline(always)]
pub unsafe fn mkdir(pathname: *const c_char, mode: mode_t) -> c_int {
    linux::mkdir(pathname, mode as linux::umode_t)
}

#[inline(always)]
pub unsafe fn lseek64(fd: c_int, offset: off64_t, whence: c_uint) -> off64_t {
    let mut result_offset: off64_t = 0;
    let result = linux::_llseek(fd, offset, &mut result_offset, whence);
    if result >= 0 {
        result_offset
    } else {
        result as off64_t
    }
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
