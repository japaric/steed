#![allow(non_camel_case_types)]

use cmp;
use linux;

pub use ctypes::*;
pub use linux::errno::*;

pub use linux::{gid_t, in_addr, in6_addr, ip_mreq, ipv6_mreq, pid_t};
pub use linux::{sa_family_t, sockaddr, sockaddr_in, sockaddr_in6};
pub use linux::{sockaddr_storage, sockaddr_un, socklen_t, stat64, suseconds_t};
pub use linux::{time_t, timespec, timeval, uid_t};

pub use linux::{AF_INET, AF_INET6, AF_UNIX};
pub use linux::{DT_BLK, DT_CHR, DT_DIR, DT_FIFO, DT_LNK, DT_REG, DT_SOCK};
pub use linux::{F_DUPFD_CLOEXEC, F_DUPFD, F_GETFL, F_SETFL};
pub use linux::{FIOCLEX, FIONBIO};
pub use linux::{IP_ADD_MEMBERSHIP, IP_DROP_MEMBERSHIP, IP_MULTICAST_LOOP};
pub use linux::{IP_MULTICAST_TTL, IP_TTL};
pub use linux::{IPV6_ADD_MEMBERSHIP, IPV6_DROP_MEMBERSHIP};
pub use linux::{IPV6_MULTICAST_LOOP, IPV6_V6ONLY};
pub use linux::{IPPROTO_IP, IPPROTO_IPV6, IPPROTO_TCP};
pub use linux::{MSG_NOSIGNAL};
pub use linux::{O_ACCMODE, O_APPEND, O_CLOEXEC, O_CREAT, O_DIRECTORY, O_EXCL};
pub use linux::{O_LARGEFILE, O_NONBLOCK, O_PATH, O_RDONLY, O_RDWR, O_TRUNC};
pub use linux::{O_WRONLY};
pub use linux::{S_IFMT, S_IFSOCK, S_IFLNK, S_IFREG, S_IFBLK, S_IFDIR, S_IFCHR};
pub use linux::{S_IFIFO};
pub use linux::{SHUT_RD, SHUT_RDWR, SHUT_WR};
pub use linux::{SO_BROADCAST, SO_ERROR, SO_RCVTIMEO, SO_REUSEADDR};
pub use linux::{SO_SNDTIMEO};
pub use linux::{SOCK_CLOEXEC, SOCK_DGRAM, SOCK_STREAM};
pub use linux::{SOL_SOCKET};
pub use linux::{SEEK_CUR, SEEK_END, SEEK_SET};
pub use linux::{TCP_NODELAY};

pub use linux::{accept, accept4, bind, close, connect, fdatasync, fstat64};
pub use linux::{fsync, ftruncate64, getpeername, getsockname, getsockopt};
pub use linux::{ioctl, link, listen, lstat64, pread64, pwrite64, read};
pub use linux::{recvfrom, rename, rmdir, send, sendto, setsockopt, socket};
pub use linux::{socketpair, shutdown, symlink, unlink, write};

pub type off64_t = i64;
pub type mode_t = u32;

// Rust 1.15.0
// src/liblibc/src/unix/notbsd/linux/mod.rs
#[cfg(issue = "22")]
pub const EAI_SYSTEM: c_int = -11;

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
