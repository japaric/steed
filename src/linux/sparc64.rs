#![allow(non_camel_case_types)]

use ctypes::*;
use linux::types::*;

use ctypes::c_int;

pub const O_APPEND: c_int = 0x0008;
pub const O_CLOEXEC: c_int = 0x400000;
pub const O_CREAT: c_int = 0x0200;
pub const O_EXCL: c_int = 0x0800;
pub const O_TRUNC: c_int = 0x0400;
