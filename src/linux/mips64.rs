use ctypes::{c_int, c_uint};

pub const O_APPEND: c_int = 0x0008;
pub const O_CLOEXEC: c_int = 0o02000000;
pub const O_CREAT: c_int = 0x0100;
pub const O_EXCL: c_int = 0x0400;
pub const O_TRUNC: c_int = 0x0200;

pub const FIOCLEX: c_uint = 0x6601;
