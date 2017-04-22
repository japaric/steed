#![allow(non_camel_case_types)]

use ctypes::*;
use linux::types::*;

use ctypes::c_int;

pub const O_APPEND: c_int = 0x0008;
pub const O_CLOEXEC: c_int = 0x400000;
pub const O_CREAT: c_int = 0x0200;
pub const O_DIRECTORY: c_int = 0o00200000;
pub const O_EXCL: c_int = 0x0800;
pub const O_NONBLOCK: c_int = 0x4000;
pub const O_PATH: c_int = 0x1000000;
pub const O_TRUNC: c_int = 0x0400;

pub const SIGCHLD: c_ulong = 17;

pub const SO_RCVTIMEO: c_int = 0x2000;
pub const SO_SNDTIMEO: c_int = 0x4000;
pub const SO_ERROR: c_int = 0x1007;
pub const SO_REUSEADDR: c_int = 0x0004;
pub const SO_BROADCAST: c_int = 0x0020;


pub const FIONBIO: c_uint = 0x8004667e;

// include/linux/net.h
pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;

pub const SOL_SOCKET: c_int = 0xffff;

pub const MAP_ANONYMOUS: c_int = 0x20;
