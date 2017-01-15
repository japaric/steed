#![allow(non_camel_case_types)]

use ctypes::{c_int, c_long, c_ushort};

// include/linux/types.h
pub type clockid_t = __kernel_clockid_t;
pub type time_t = __kernel_time_t;
pub type umode_t = c_ushort;

// include/uapi/asm-generic/posix_types.h
type __kernel_clockid_t = c_int;
type __kernel_long_t = c_long;
type __kernel_time_t = __kernel_long_t;

// include/uapi/linux/time.h
#[derive(Clone, Copy)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}
