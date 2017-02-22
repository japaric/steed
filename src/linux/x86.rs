#![allow(non_camel_case_types)]

use ctypes::*;
use linux::types::*;

pub const O_APPEND: c_int = 0o00002000;
pub const O_CLOEXEC: c_int = 0o02000000;
pub const O_CREAT: c_int = 0o00000100;
pub const O_DIRECTORY: c_int = 0o00200000;
pub const O_EXCL: c_int = 0o00000200;
pub const O_LARGEFILE: c_int = 0x8000;
pub const O_NONBLOCK: c_int = 0o00004000;
pub const O_PATH: c_int = 0o010000000;
pub const O_TRUNC: c_int = 0o00001000;

pub const FIOCLEX: c_uint = 0x5451;
pub const FIONBIO: c_uint = 0x5421;

// include/uapi/asm-generic/socket.h
pub const SO_RCVTIMEO: c_int = 20;
pub const SO_SNDTIMEO: c_int = 21;
pub const SO_ERROR: c_int = 4;
pub const SO_REUSEADDR: c_int = 2;
pub const SO_BROADCAST: c_int = 6;

pub const SIGCHLD: c_ulong = 17;

// include/linux/net.h
pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;

pub const SOL_SOCKET: c_int = 1;

pub const MAP_ANONYMOUS: c_int = 0x20;

// include/uapi/linux/net.h
pub const SYS_SOCKET: c_ulong = 1;
pub const SYS_BIND: c_ulong = 2;
pub const SYS_CONNECT: c_ulong = 3;
pub const SYS_LISTEN: c_ulong = 4;
pub const SYS_ACCEPT: c_ulong = 5;
pub const SYS_GETSOCKNAME: c_ulong = 6;
pub const SYS_GETPEERNAME: c_ulong = 7;
pub const SYS_SOCKETPAIR: c_ulong = 8;
// pub const SYS_SEND: c_ulong =        9;
// pub const SYS_RECV: c_ulong =        10;
pub const SYS_SENDTO: c_ulong = 11;
pub const SYS_RECVFROM: c_ulong = 12;
pub const SYS_SHUTDOWN: c_ulong = 13;
pub const SYS_SETSOCKOPT: c_ulong = 14;
pub const SYS_GETSOCKOPT: c_ulong = 15;
// pub const SYS_SENDMSG: c_ulong =     16;
// pub const SYS_RECVMSG: c_ulong =     17;
pub const SYS_ACCEPT4: c_ulong = 18;
// pub const SYS_RECVMMSG: c_ulong =    19;
// pub const SYS_SENDMMSG: c_ulong =    20;

// include/linux/types.h
pub type ino_t = __kernel_ino_t;
// include/uapi/asm-generic/posix_types.h
type __kernel_ino_t = __kernel_long_t;
type __kernel_long_t = c_long;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct stat64 {
    pub st_dev: dev_t,
    __pad1: c_uint,
    __st_ino: ino_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    __pad2: c_uint,
    pub st_size: off64_t,
    pub st_blksize: blksize_t,
    pub st_blocks: blkcnt64_t,
    pub st_atime: time_t,
    pub st_atime_nsec: c_long,
    pub st_mtime: time_t,
    pub st_mtime_nsec: c_long,
    pub st_ctime: time_t,
    pub st_ctime_nsec: c_long,
    pub st_ino: ino64_t,
}

pub type blksize_t = i32;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct user_desc {
    pub entry_number: c_uint,
    pub base_addr: c_uint,
    pub limit: c_uint,
    pub flags: c_uint,
}

// arch/x86/kernel/tls.c
#[inline(always)]
pub unsafe fn set_thread_area(u_info: *mut user_desc) -> ssize_t {
    // TODO(steed)!
    syscall!(SET_THREAD_AREA, u_info) as ssize_t
}
