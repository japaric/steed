//! https://w3challs.com/syscalls/?arch=powerpc_32

use ctypes::c_int;

pub const O_APPEND: c_int = 0o00002000;
pub const O_CLOEXEC: c_int = 0o02000000;
pub const O_CREAT: c_int = 0o00000100;
pub const O_EXCL: c_int = 0o00000200;
pub const O_TRUNC: c_int = 0o00001000;
