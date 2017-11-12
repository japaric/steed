#![cfg_attr(test, feature(rustc_private))]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(allow_internal_unstable)]
#![feature(asm)]
#![feature(box_syntax)]
#![feature(cfg_target_vendor)]
#![feature(collections_range)]
#![feature(compiler_builtins_lib)]
#![feature(const_atomic_bool_new)]
#![feature(const_atomic_u8_new)]
#![feature(const_atomic_usize_new)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(custom_attribute)]
#![feature(dropck_eyepatch)]
#![feature(dropck_parametricity)]
#![feature(exact_size_is_empty)]
#![feature(fnbox)]
#![feature(fused)]
#![feature(generic_param_attrs)]
#![feature(global_asm)]
#![feature(int_error_internals)]
#![feature(integer_atomics)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(naked_functions)]
#![feature(needs_drop)]
#![feature(optin_builtin_traits)]
#![feature(placement_new_protocol)]
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
extern crate alloc as core_collections;
extern crate compiler_builtins;
#[cfg(not(test))]
#[cfg(feature = "ralloc")]
extern crate ralloc;
#[cfg(not(test))]
#[cfg(feature = "naive_ralloc")]
// this contains the global allocator (static variable)
#[allow(unused_extern_crates)]
extern crate naive_ralloc;
#[macro_use]
extern crate sc;
extern crate std_unicode;
#[cfg(test)]
extern crate test;

// Rust 1.16.0
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

// Rust 1.16.0
#[macro_use]
mod macros;

pub mod sync;

// Rust 1.16.0
pub mod ascii;
// Rust nightly 1.21.0 c11f689d2 (incomplete)
pub mod collections;
// Rust 1.16.0 (incomplete)
pub mod env;
// Rust 1.16.0
pub mod error;
// Rust 1.16.0
pub mod ffi;
// Rust 1.16.0 (no tests)
pub mod fs;
// Rust 1.16.0 (no tests, mostly, not the submodules `lazy`, `stdio`)
pub mod io;
// Rust 1.16.0 (no tests, missing `lookup_host` and friends)
pub mod net;
// Rust 1.16.0
pub mod num;
// Rust 1.16.0 (no tests)
pub mod os;
// Rust 1.16.0
pub mod path;
// Rust 1.16.0 (incomplete)
pub mod process;
pub mod thread;
// Rust 1.16.0
pub mod time;

// Rust 1.16.0 (plus our own entry point)
// The runtime entry point and a few unstable public functions used by the
// compiler
pub mod rt;

mod ctypes;
mod linux;
// Rust 1.16.0
mod memchr;
#[cfg(not(test))]
mod panicking;
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
