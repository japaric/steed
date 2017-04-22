#![allow(non_camel_case_types)]

use ctypes::*;
use linux::types::*;

pub const O_APPEND: c_int = 0x0008;
pub const O_CLOEXEC: c_int = 0o02000000;
pub const O_CREAT: c_int = 0x0100;
pub const O_DIRECTORY: c_int = 0o00200000;
pub const O_EXCL: c_int = 0x0400;
pub const O_LARGEFILE: c_int = 0x2000;
pub const O_NONBLOCK: c_int = 0x0080;
pub const O_PATH: c_int = 0o010000000;
pub const O_TRUNC: c_int = 0x0200;

pub const FIOCLEX: c_uint = 0x6601;
pub const FIONBIO: c_uint = 0x667e;

pub const SO_RCVTIMEO: c_int = 0x1006;
pub const SO_SNDTIMEO: c_int = 0x1005;
pub const SO_ERROR: c_int = 0x1007;
pub const SO_REUSEADDR: c_int = 0x0004;
pub const SO_BROADCAST: c_int = 0x0020;

pub const SIGCHLD: c_ulong = 18;

// arch/mips/include/asm/socket.h
pub const SOCK_DGRAM: c_int = 1;
pub const SOCK_STREAM: c_int = 2;

pub const SOL_SOCKET: c_int = 0xffff;

pub const MAP_ANONYMOUS: c_int = 0x0800;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct stat64 {
    pub st_dev: c_ulong,
    __st_pad1: [c_long; 3],
    pub st_ino: ino64_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: c_ulong,
    __st_pad2: [c_long; 2],
    pub st_size: off64_t,
    pub st_atime: time_t,
    pub st_atime_nsec: c_long,
    pub st_mtime: time_t,
    pub st_mtime_nsec: c_long,
    pub st_ctime: time_t,
    pub st_ctime_nsec: c_long,
    pub st_blksize: blksize_t,
    __st_pad3: c_long,
    pub st_blocks: blkcnt64_t,
    __st_pad5: [c_long; 14],
}

pub type blksize_t = i32;
