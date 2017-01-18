// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Runtime services
//!
//! The `rt` module provides a narrow set of runtime services,
//! including the global heap (exported in `heap`) and unwinding and
//! backtrace support. The APIs in this module are highly unstable,
//! and should be considered as private implementation details for the
//! time being.

#![unstable(feature = "rt",
            reason = "this public module should not exist and is highly likely \
                      to disappear",
            issue = "0")]
#![doc(hidden)]

#[cfg(issue = "12")]
// Reexport some of our utilities which are expected by other crates.
pub use panicking::{begin_panic, begin_panic_fmt, update_panic_count};

// This is how the `start` lang item actually works:
//
// When compiling an executable, the compiler will create and inject into the
// executable an unmangled `main` function (symbol) whose definition looks like
// this:
//
// ``` rust
// #[no_mangle]
// pub fn main(argc: isize, argv: *const *const u8) -> isize {
//     start(user_main as *const u8, argc, argv)
// }
// ```
//
// Where `start` is *this* `start` lang item and `user_main` is the (mangled)
// `main` function within the executable source code.
#[lang = "start"]
fn lang_start(main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    use core::mem;

    unsafe {
        (mem::transmute::<_, fn()>(main))();
    }
    0
}

// This is the REAL entry point of all programs
#[cfg_attr(any(target_arch = "mips",
               target_arch = "mips64"), export_name = "__start")]
#[cfg_attr(any(target_arch = "aarch64",
               target_arch = "arm",
               target_arch = "powerpc",
               target_arch = "powerpc64",
               target_arch = "sparc64"), export_name = "_start")]
#[cfg_attr(any(target_arch = "x86",
               target_arch = "x86_64"), export_name = "_start_rust")]
#[stable(feature = "steed", since = "1.0.0")]
pub extern "C" fn start() -> ! {
    use core::ptr;

    extern "C" {
        // This is the (unmangled) `main` function that the compiler synthesizes
        // See the `start` lang item above
        fn main(argc: isize, argv: *const *const u8) -> isize;
    }

    // TODO #7 program arguments
    unsafe { ::linux::exit(main(0, ptr::null()) as i32) }
}

// NOTE needed to get a 16 byte aligned stack. Without this, programs segfault
// when executing SSE instructions like `movaps` or `movdqa`
#[cfg(any(target_arch = "x86",
          target_arch = "x86_64"))]
#[export_name = "_start"]
#[stable(feature = "steed", since = "1.0.0")]
#[naked]
pub fn entry() -> ! {
    use core::intrinsics;

    unsafe {
        asm!("call _start_rust");
        intrinsics::unreachable()
    }
}

#[cfg(issue = "14")]
#[cfg(not(test))]
#[lang = "start"]
fn lang_start(main: *const u8, argc: isize, argv: *const *const u8) -> isize {
    use mem;
    use panic;
    use sys;
    use sys_common;
    use sys_common::thread_info::{self, NewThread};
    use thread::Thread;

    sys::init();

    let failed = unsafe {
        let main_guard = sys::thread::guard::init();
        sys::stack_overflow::init();

        // Next, set up the current Thread with the guard information we just
        // created. Note that this isn't necessary in general for new threads,
        // but we just do this to name the main thread and to give it correct
        // info about the stack bounds.
        let thread: Thread = NewThread::new(Some("main".to_owned()));
        thread_info::set(main_guard, thread);

        // Store our args if necessary in a squirreled away location
        sys::args::init(argc, argv);

        // Let's run some code!
        let res = panic::catch_unwind(mem::transmute::<_, fn()>(main));
        sys_common::cleanup();
        res.is_err()
    };

    if failed { 101 } else { 0 }
}
