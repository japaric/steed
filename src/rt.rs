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

use core::intrinsics;

// The bottom of the program's stack
//
// Most (all?) architectures grow their stack towards smaller address so the
// bottom of the stack actually has the largest, valid memory address.
#[repr(C)]
pub struct Stack {
    argc: isize,
    // NOTE Null terminated string
    argv0: &'static u8,
}

impl Stack {
    fn argc(&self) -> isize {
        self.argc
    }

    fn argv(&self) -> *const *const u8 {
        &self.argv0 as *const &'static u8 as *const *const u8
    }
}

// This is the entry point of all programs or, IOW, this is where all programs
// begin their execution.
//
// At this point we can get a pointer to the bottom of the stack. The ELF loader
// has set up useful information in there, like program arguments and
// environment variables, so it's important to retrieve that value. The value
// will be stored in an architecture specific register.
//
// We can't do much at this state because the stack is set up in a way that's
// incompatible with most operations so we have no choice but to call another
// function.
//
// So this what we'll do, expressed in Rust code:
//
// ``` rust
// start($BSP)
// ```
//
// Call some `start` function whose argument is the pointer to the bottom of the
// stack (`$BSP`).
//
// Most Rust operations, like using a local/stack variable, will modify the
// register where `$BSP` resides so we have to use assembly to avoid that.

#[cfg(target_arch = "x86_64")]
#[export_name = "_start"]
#[naked]
pub extern "C" fn entry() -> ! {
    unsafe {
        asm!("mov %rsp, %rdi
              call _start_rust"
             :
             :
             :
             : "volatile");
        intrinsics::unreachable()
    }
}

#[cfg(target_arch = "x86")]
#[export_name = "_start"]
#[naked]
pub extern "C" fn entry() -> ! {
    unsafe {
        asm!("mov %esp, %eax
              push %eax
              push %eax
              call 1f
              1: push %eax
              call _start_rust"
             :
             :
             :
             : "volatile");
        intrinsics::unreachable()
    }
}

#[cfg(target_arch = "arm")]
#[export_name = "_start"]
#[naked]
pub extern "C" fn entry() -> ! {
    unsafe {
        asm!("mov r0, sp
              b _start_rust"
             :
             :
             :
             : "volatile");
        intrinsics::unreachable()
    }
}

#[cfg(target_arch = "aarch64")]
#[export_name = "_start"]
#[naked]
pub extern "C" fn entry() -> ! {
    unsafe {
        asm!("mov x0, sp
              b _start_rust"
             :
             :
             :
             : "volatile");
        intrinsics::unreachable()
    }
}

#[cfg(target_arch = "mips")]
#[export_name = "__start"]
#[naked]
pub extern "C" fn entry() -> ! {
    unsafe {
        asm!("move $$4, $$sp
              jal _start_rust"
             :
             :
             :
             : "volatile");
        intrinsics::unreachable()
    }
}

// FIXME
#[cfg(target_arch = "mips64")]
#[export_name = "__start"]
#[naked]
pub extern "C" fn entry() -> ! {
    unsafe {
        asm!("move $$4, $$sp
              jal _start_rust"
             :
             :
             :
             : "volatile");
        intrinsics::unreachable()
    }
}

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
#[export_name = "_start"]
#[naked]
pub extern "C" fn entry() -> ! {
    unsafe {
        asm!("mr 3, 1
              b _start_rust"
             :
             :
             :
             : "volatile");
        intrinsics::unreachable()
    }
}

// Now we have left behind the entry point and we can now perform pretty much
// any operation without worries. We also have a pointer to the bottom of the
// stack from which we can retrieve the program arguments and other things.
//
// Eventually we'll have to call the `main` function which, as you may have
// guess, is related to the `fn main() { .. }` function one writes in their
// crates. But the relationship is not that simple due to the presence of the
// `start` lang item. The next section explains how that lang item works.
#[inline(never)]
#[export_name = "_start_rust"]
pub extern "C" fn start(sp: &'static Stack) -> ! {
    extern "C" {
        fn main(argc: isize, argv: *const *const u8) -> isize;
    }

    unsafe { ::linux::exit_group(main(sp.argc(), sp.argv()) as i32) }
}

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
extern "C" fn lang_start(main: *const u8,
                         _argc: isize,
                         _argv: *const *const u8)
                         -> isize {
    use core::mem;

    unsafe {
        (mem::transmute::<_, fn()>(main))();
    }

    0
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
