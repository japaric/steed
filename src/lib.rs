#![feature(asm)]
#![feature(collections)]
#![feature(compiler_builtins_lib)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(naked_functions)]
#![no_std]

#[macro_reexport(vec)]
extern crate collections as core_collections;
extern crate compiler_builtins;
extern crate ralloc;
#[macro_use]
extern crate sc;

use core::intrinsics;

pub use core_collections::vec;

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
