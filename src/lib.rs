#![feature(alloc)]
#![feature(allow_internal_unstable)]
#![feature(asm)]
#![feature(cfg_target_vendor)]
#![feature(collections)]
#![feature(collections_bound)]
#![feature(collections_range)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(custom_attribute)]
#![feature(dropck_parametricity)]
#![feature(fused)]
#![feature(heap_api)]
#![feature(int_error_internals)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(naked_functions)]
#![feature(oom)]
#![feature(prelude_import)]
#![feature(rand)]
#![feature(raw)]
#![feature(shared)]
#![feature(sip_hash_13)]
#![feature(slice_concat_ext)]
#![feature(slice_patterns)]
#![feature(staged_api)]
#![feature(str_internals)]
#![feature(try_from)]
#![feature(unicode)]
#![feature(unique)]
#![feature(zero_one)]
#![no_std]

#![stable(feature = "rust1", since = "1.0.0")]

// Explicitly import the prelude. The compiler uses this same unstable attribute
// to import the prelude implicitly when building crates that depend on std.
#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

// We want to reexport a few macros from core but libcore has already been
// imported by the compiler (via our #[no_std] attribute) In this case we just
// add a new crate name so we can attach the reexports to it.
#[macro_reexport(assert, assert_eq, assert_ne, debug_assert, debug_assert_eq,
                 debug_assert_ne, panic, unreachable, unimplemented, write,
                 writeln, try)]
extern crate core as __core;

extern crate alloc;
#[macro_use]
#[macro_reexport(vec, format)]
extern crate collections as core_collections;
extern crate compiler_builtins;
#[cfg(not(test))]
#[cfg(feature = "ralloc")]
extern crate ralloc;
#[cfg(not(test))]
#[cfg(feature = "naive_ralloc")]
extern crate naive_ralloc;
#[macro_use]
extern crate sc;
extern crate std_unicode;
extern crate rand as core_rand;
#[cfg(test)]
extern crate test;

// Rust 1.15.0
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

// Rust 1.15.0
#[macro_use]
mod macros;

pub mod sync;

// Rust 1.15.0
pub mod ascii;
// Rust 1.15.0
pub mod collections;
// Rust 1.15.0
pub mod error;
// Rust 1.15.0
pub mod ffi;
// Rust 1.15.0
pub mod fs;
// Rust 1.15.0 (mostly, not the submodules `lazy`, `stdio`)
pub mod io;
// Rust 1.15.0
pub mod memchr;
// Rust 1.15.0
pub mod num;
// Rust 1.15.0
pub mod net;
// Rust 1.15.0
pub mod os;
// Rust 1.15.0
pub mod path;
// Rust 1.15.0
pub mod process;
// Rust 1.15.0
pub mod time;

// Rust 1.15.0
// The runtime entry point and a few unstable public functions used by the
// compiler
pub mod rt;

mod ctypes;
mod linux;
#[cfg(not(test))]
mod panicking;
mod rand;
mod sys;
mod sys_common;
mod libc;

// NOTE These two are "undefined" symbols that LLVM emits but that, AFAIK, we
// never use
#[cfg(not(test))]
#[doc(hidden)]
#[no_mangle]
#[unstable(feature = "steed", issue = "0")]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr0() {
    intrinsics::unreachable()
}

#[cfg(not(test))]
#[doc(hidden)]
#[no_mangle]
#[unstable(feature = "steed", issue = "0")]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr1() {
    intrinsics::unreachable()
}
