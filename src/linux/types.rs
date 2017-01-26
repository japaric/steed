#![allow(non_camel_case_types)]

use ctypes::*;

pub use super::arch::stat64;

// include/linux/types.h
pub type clockid_t = __kernel_clockid_t;
//pub type dev_t = __kernel_dev_t;
pub type loff_t = __kernel_loff_t;
pub type mode_t = __kernel_mode_t;
pub type nlink_t = u32;
pub type time_t = __kernel_time_t;
pub type umode_t = c_ushort;

// include/uapi/asm-generic/posix_types.h
type __kernel_clockid_t = c_int;
//type __kernel_dev_t = u32;
type __kernel_gid_t = c_uint;
type __kernel_loff_t = c_longlong;
type __kernel_long_t = c_long;
type __kernel_mode_t = c_uint;
type __kernel_off64_t = c_longlong;
type __kernel_time_t = __kernel_long_t;
type __kernel_uid_t = c_uint;

// include/uapi/linux/time.h
#[derive(Clone, Copy)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

// include/linux/dirent.h
#[derive(Clone, Copy)]
#[repr(C)]
pub struct linux_dirent64 {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: c_ushort,
    pub d_type: c_uchar,
    pub d_name: [c_char; 0],
}

// Where from?
pub type blkcnt64_t = i64;
#[cfg(not(any(target_arch = "mips", target_arch = "mips64")))] pub type dev_t = u64;
pub type gid_t = __kernel_gid_t;
pub type ino64_t = c_longlong;
pub type off64_t = __kernel_off64_t;
pub type uid_t = __kernel_uid_t;
