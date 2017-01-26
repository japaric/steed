#![allow(non_camel_case_types)]

use ctypes::*;
use linux::types::*;

pub const O_APPEND: c_int = 0x0008;
pub const O_CLOEXEC: c_int = 0o02000000;
pub const O_CREAT: c_int = 0x0100;
pub const O_DIRECTORY: c_int = 0o00200000;
pub const O_EXCL: c_int = 0x0400;
pub const O_TRUNC: c_int = 0x0200;

pub const FIOCLEX: c_uint = 0x6601;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct stat64 {
    pub st_dev: c_ulong,
    __st_pad1: [c_long; 2],
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
    __st_pad5: [c_long; 7],
}

pub type blksize_t = i64;
