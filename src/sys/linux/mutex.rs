// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::parking_lot;

pub struct Mutex { inner: parking_lot::RawMutex }

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

#[allow(dead_code)] // sys isn't exported yet
impl Mutex {
    pub const fn new() -> Mutex {
        // Might be moved and address is changing it is better to avoid
        // initialization of potentially opaque OS data before it landed
        Mutex { inner: parking_lot::RawMutex::new() }
    }
    #[inline]
    pub unsafe fn init(&mut self) {
    }
    #[inline]
    pub unsafe fn lock(&self) {
        self.inner.lock()
    }
    #[inline]
    pub unsafe fn unlock(&self) {
        self.inner.unlock(false)
    }
    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        self.inner.try_lock()
    }
    #[inline]
    pub unsafe fn destroy(&self) {
    }
}

/*
pub struct ReentrantMutex { inner: UnsafeCell<libc::pthread_mutex_t> }

unsafe impl Send for ReentrantMutex {}
unsafe impl Sync for ReentrantMutex {}

impl ReentrantMutex {
    pub unsafe fn uninitialized() -> ReentrantMutex {
        ReentrantMutex { inner: mem::uninitialized() }
    }

    pub unsafe fn init(&mut self) {
        let mut attr: libc::pthread_mutexattr_t = mem::uninitialized();
        let result = libc::pthread_mutexattr_init(&mut attr as *mut _);
        debug_assert_eq!(result, 0);
        let result = libc::pthread_mutexattr_settype(&mut attr as *mut _,
                                                    libc::PTHREAD_MUTEX_RECURSIVE);
        debug_assert_eq!(result, 0);
        let result = libc::pthread_mutex_init(self.inner.get(), &attr as *const _);
        debug_assert_eq!(result, 0);
        let result = libc::pthread_mutexattr_destroy(&mut attr as *mut _);
        debug_assert_eq!(result, 0);
    }

    pub unsafe fn lock(&self) {
        let result = libc::pthread_mutex_lock(self.inner.get());
        debug_assert_eq!(result, 0);
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        libc::pthread_mutex_trylock(self.inner.get()) == 0
    }

    pub unsafe fn unlock(&self) {
        let result = libc::pthread_mutex_unlock(self.inner.get());
        debug_assert_eq!(result, 0);
    }

    pub unsafe fn destroy(&self) {
        let result = libc::pthread_mutex_destroy(self.inner.get());
        debug_assert_eq!(result, 0);
    }
}
*/
