#![feature(allocator)]
#![feature(core_intrinsics)]
#![feature(linkage)]
#![no_std]
#![allocator]

#[macro_use]
extern crate sc;

use core::intrinsics;

// fs/mmap.c
pub unsafe fn brk(brk: usize) -> usize {
    syscall!(BRK, brk)
}

fn allocate(size: usize) -> *mut u8 {
    let align = 16;
    let mask = align - 1;
    unsafe {
        let cur = brk(0);
        let aligned = (cur + mask) & !mask;
        let new = aligned + size;
        let actual = brk(new);
        if actual < new {
            intrinsics::abort();
        } else {
            aligned as *mut u8
        }
    }
}

#[linkage = "external"]
#[no_mangle]
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    allocate(size)
}

#[linkage = "external"]
#[no_mangle]
pub extern fn __rust_allocate_zeroed(size: usize, align: usize) -> *mut u8 {
    unsafe {
        let result = __rust_allocate(size, align);
        intrinsics::write_bytes(result, 0, size);
        result
    }
}

#[linkage = "external"]
#[no_mangle]
pub extern fn __rust_deallocate(_ptr: *mut u8, _old_size: usize, _align: usize) {
}

#[linkage = "external"]
#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, old_size: usize, size: usize,
                                _align: usize) -> *mut u8 {
    if size <= old_size {
        return ptr;
    }
    let new = allocate(size);
    if new.is_null() {
        return new;
    }
    unsafe {
        for i in 0..old_size as isize {
            *new.offset(i) = *ptr.offset(i);
        }
    }
    __rust_deallocate(ptr, old_size, _align);
    new
}

#[linkage = "external"]
#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize,
                                        _size: usize, _align: usize) -> usize {
    old_size // this api is not supported by naive_ralloc
}

#[linkage = "external"]
#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}
