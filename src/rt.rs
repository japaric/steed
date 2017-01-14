//! Entry point

#![unstable(feature = "steed", issue = "0")]

use ctypes::c_int;
use linux;

#[unstable(feature = "steed", issue = "0")]
#[cfg_attr(any(target_arch = "mips",
               target_arch = "mips64"), export_name = "__start")]
#[cfg_attr(any(target_arch = "aarch64",
               target_arch = "arm",
               target_arch = "powerpc",
               target_arch = "powerpc64",
               target_arch = "sparc64"), export_name = "_start")]
#[cfg_attr(any(target_arch = "x86",
               target_arch = "x86_64"), export_name = "_start_rust")]
pub extern "C" fn start() -> ! {
    extern "Rust" {
        fn main() -> c_int;
    }

    unsafe { linux::exit(main()) }
}

#[unstable(feature = "steed", issue = "0")]
// NOTE needed to get a 16 byte aligned stack. Without this, programs segfault
// when executing SSE instructions like `movaps` or `movdqa`
#[cfg(any(target_arch = "x86",
          target_arch = "x86_64"))]
#[export_name = "_start"]
#[naked]
pub fn entry() -> ! {
    use core::intrinsics;

    unsafe {
        asm!("call _start_rust");
        intrinsics::unreachable()
    }
}
