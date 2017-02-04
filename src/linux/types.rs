#![allow(non_camel_case_types)]

use ctypes::*;

pub use super::arch::stat64;

// include/linux/types.h
pub type clockid_t = __kernel_clockid_t;
// pub type dev_t = __kernel_dev_t;
pub type loff_t = __kernel_loff_t;
pub type mode_t = __kernel_mode_t;
pub type nlink_t = u32;
pub type pid_t = __kernel_pid_t;
pub type time_t = __kernel_time_t;
pub type umode_t = c_ushort;
pub type suseconds_t = __kernel_suseconds_t;

// include/linux/socket.h
pub type sa_family_t = __kernel_sa_family_t;
// include/uapi/linux/socket.h
type __kernel_sa_family_t = c_ushort;

// include/uapi/asm-generic/posix_types.h
type __kernel_clockid_t = c_int;
// type __kernel_dev_t = u32;
type __kernel_gid_t = c_uint;
type __kernel_loff_t = c_longlong;
type __kernel_long_t = c_long;
type __kernel_mode_t = c_uint;
type __kernel_off64_t = c_longlong;
type __kernel_pid_t = c_int;
type __kernel_time_t = __kernel_long_t;
type __kernel_uid_t = c_uint;
type __kernel_suseconds_t = __kernel_long_t;

// include/uapi/linux/time.h
#[derive(Clone, Copy)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

// include/uapi/linux/time.h
#[derive(Clone, Copy)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
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

pub struct rusage {
    _unimplemented: (),
}

// Where from?
pub type blkcnt64_t = i64;
#[cfg(not(any(target_arch = "mips", target_arch = "mips64")))]
pub type dev_t = u64;
pub type gid_t = __kernel_gid_t;
pub type ino64_t = c_longlong;
pub type off64_t = __kernel_off64_t;
pub type uid_t = __kernel_uid_t;

// libc helper type
pub type socklen_t = c_int;

// include/linux/socket.h
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}

// include/uapi/linux/in.h
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub pad: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

// include/uapi/linux/in.h
#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr {
    pub s_addr: u32,
}

// include/uapi/linux/in6.h
#[repr(C)]
#[derive(Clone, Copy)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
    __align: [u32; 0],
}

// include/uapi/linux/in.h
#[repr(C)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}

// include/uapi/linux/in6.h
#[repr(C)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: c_int,
}
