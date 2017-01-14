#![feature(alloc)]
#![feature(asm)]
#![feature(collections)]
#![feature(compiler_builtins_lib)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(naked_functions)]
#![feature(raw)]
#![feature(unicode)]
#![no_std]

extern crate alloc;
#[macro_reexport(vec)]
extern crate collections as core_collections;
extern crate compiler_builtins;
extern crate ralloc;
#[macro_use]
extern crate sc;
extern crate std_unicode;

// Public module declarations and reexports
pub use core::any;
pub use core::cell;
pub use core::clone;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::hash;
pub use core::intrinsics;
pub use core::iter;
pub use core::marker;
pub use core::mem;
pub use core::ops;
pub use core::ptr;
pub use core::raw;
pub use core::result;
pub use core::option;
pub use core::isize;
pub use core::i8;
pub use core::i16;
pub use core::i32;
pub use core::i64;
pub use core::usize;
pub use core::u8;
pub use core::u16;
pub use core::u32;
pub use core::u64;
pub use alloc::boxed;
pub use alloc::rc;
pub use core_collections::borrow;
pub use core_collections::fmt;
pub use core_collections::slice;
pub use core_collections::str;
pub use core_collections::string;
pub use core_collections::vec;
pub use std_unicode::char;

#[macro_use]
mod macros;

pub mod fs;
pub mod io;
pub mod process;

#[doc(hidden)]
pub mod rt;

mod ctypes;
mod linux;
mod panicking;

// NOTE These two are "undefined" symbols that LLVM emits but that, AFAIK, we
// never use
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr0() {
    intrinsics::unreachable()
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr1() {
    intrinsics::unreachable()
}
