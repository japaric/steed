#![feature(alloc)]
#![feature(allocator_api)]
#![feature(global_allocator)]
#![no_std]

extern crate alloc;
#[macro_use]
extern crate sc;

use alloc::allocator::{Alloc, AllocErr, Layout};

struct Allocator;

unsafe impl<'a> Alloc for &'a Allocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let size = layout.size();
        let align = layout.align();
        let mask = align - 1;

        let cur = brk(0);
        let aligned = (cur + mask) & !mask;
        let new = aligned + size;
        let actual = brk(new);
        if actual < new {
            Err(AllocErr::Exhausted { request: layout })
        } else {
            Ok(aligned as *mut u8)
        }
    }

    unsafe fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
        // no deallocation
    }
}

#[global_allocator]
static GLOBAL: Allocator = Allocator;

// fs/mmap.c
pub unsafe fn brk(brk: usize) -> usize {
    syscall!(BRK, brk)
}
