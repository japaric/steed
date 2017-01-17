#![feature(alloc)]
#![feature(allow_internal_unstable)]
#![feature(asm)]
#![feature(collections)]
#![feature(collections_bound)]
#![feature(collections_range)]
#![feature(compiler_builtins_lib)]
#![feature(core_intrinsics)]
#![feature(dropck_parametricity)]
#![feature(fused)]
#![feature(int_error_internals)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(naked_functions)]
#![feature(prelude_import)]
#![feature(raw)]
#![feature(slice_concat_ext)]
#![feature(staged_api)]
#![feature(try_from)]
#![feature(unicode)]
#![feature(zero_one)]
#![no_std]

#![stable(feature = "rust1", since = "1.0.0")]

// Explicitly import the prelude. The compiler uses this same unstable attribute
// to import the prelude implicitly when building crates that depend on std.
#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

extern crate alloc;
#[macro_reexport(vec)]
extern crate collections as core_collections;
extern crate compiler_builtins;
extern crate ralloc;
#[macro_use]
extern crate sc;
extern crate std_unicode;

// Rust 1.14.0
// The Rust prelude
pub mod prelude;

// Public module declarations and reexports
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::any;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::cell;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::clone;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::cmp;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::convert;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::default;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::hash;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::intrinsics;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::iter;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::marker;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::mem;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::ops;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::ptr;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::raw;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::result;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::option;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::isize;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i8;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i16;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i32;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i64;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::usize;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u8;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u16;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u32;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u64;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::boxed;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::rc;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core_collections::borrow;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core_collections::fmt;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core_collections::slice;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core_collections::str;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core_collections::string;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core_collections::vec;
#[stable(feature = "rust1", since = "1.0.0")]
pub use std_unicode::char;

// Rust 1.14.0
#[macro_use]
mod macros;

pub mod entry;
pub mod fs;
pub mod io;
pub mod process;

// Rust 1.14.0
pub mod ascii;
// Rust 1.14.0
pub mod collections;
// Rust 1.14.0
pub mod error;
// Rust 1.14.0
pub mod ffi;
// Rust 1.14.0
pub mod num;
// Rust 1.14.0
pub mod memchr;
// Rust 1.14.0
pub mod time;

// Rust 1.14.0
// The runtime entry point and a few unstable public functions used by the
// compiler
pub mod rt;

mod ctypes;
mod linux;
mod panicking;
mod sys;

// NOTE These two are "undefined" symbols that LLVM emits but that, AFAIK, we
// never use
#[unstable(feature = "steed", issue = "0")]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr0() {
    intrinsics::unreachable()
}

#[unstable(feature = "steed", issue = "0")]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr1() {
    intrinsics::unreachable()
}
