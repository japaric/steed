#![allow(non_camel_case_types)]

use ctypes::*;
use linux::types::*;

pub const O_APPEND: c_int = 0o00002000;
pub const O_CLOEXEC: c_int = 0o02000000;
pub const O_CREAT: c_int = 0o00000100;
pub const O_DIRECTORY: c_int = 0o00200000;
pub const O_EXCL: c_int = 0o00000200;
pub const O_LARGEFILE: c_int = 0;
pub const O_PATH: c_int = 0o010000000;
pub const O_TRUNC: c_int = 0o00001000;

pub const FIOCLEX: c_uint = 0x5451;

pub const SIGCHLD: c_ulong = 17;

pub type blksize_t = i64;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct stat64 {
    pub st_dev: dev_t,
    pub st_ino: ino64_t,
    pub st_nlink: nlink_t,
    pub st_mode: mode_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    __pad0: c_int,
    pub st_rdev: dev_t,
    pub st_size: off64_t,
    pub st_blksize: blksize_t,
    pub st_blocks: blkcnt64_t,
    pub st_atime: time_t,
    pub st_atime_nsec: c_long,
    pub st_mtime: time_t,
    pub st_mtime_nsec: c_long,
    pub st_ctime: time_t,
    pub st_ctime_nsec: c_long,
    __reserved: [c_long; 3],
}
