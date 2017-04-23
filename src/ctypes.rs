#![allow(non_camel_case_types)]
#![allow(unused)]

#[cfg(any(target_arch = "aarch64",
          target_arch = "arm",
          target_arch = "powerpc",
          target_arch = "powerpc64"))]
pub type c_char = u8;

#[cfg(any(target_arch = "mips",
          target_arch = "mips64",
          target_arch = "sparc64",
          target_arch = "x86",
          target_arch = "x86_64"))]
pub type c_char = i8;

#[cfg(target_pointer_width = "32")] pub type c_long = i32;
#[cfg(target_pointer_width = "64")] pub type c_long = i64;
#[cfg(target_pointer_width = "32")] pub type c_ulong = u32;
#[cfg(target_pointer_width = "64")] pub type c_ulong = u64;

pub type c_uchar = u8;
pub type c_schar = i8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type c_float = f32;
pub type c_double = f64;

pub type size_t = usize;
pub type ssize_t = isize;

pub type c_void = c_char;
