// Copyright 2013-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use mem;
use slice;

pub fn hashmap_random_keys() -> (u64, u64) {
    let mut v = (0, 0);
    unsafe {
        let view = slice::from_raw_parts_mut(&mut v as *mut _ as *mut u8,
                                             mem::size_of_val(&v));
        imp::fill_bytes(view);
    }
    return v
}

mod imp {
    use ctypes::{c_char, ssize_t};
    use fs::File;
    use io::{ErrorKind, Read};
    use linux::{self, errno};
    use sys::cvtu;

    fn getrandom(buf: &mut [u8]) -> ssize_t {
        unsafe {
            linux::getrandom(buf.as_mut_ptr() as *mut c_char,
                             buf.len(),
                             linux::GRND_NONBLOCK)
        }
    }

    fn getrandom_fill_bytes(v: &mut [u8]) -> bool {
        let mut read = 0;
        while read < v.len() {
            match cvtu(getrandom(&mut v[read..])) {
                Err(e) => {
                    let kind = e.kind();
                    if kind == ErrorKind::Interrupted {
                        continue;
                    } else if kind == ErrorKind::WouldBlock {
                        return false
                    } else {
                        panic!("unexpected getrandom error: {}", e);
                    }
                }
                Ok(result) => read += result,
            }
        }

        return true
    }

    fn is_getrandom_available() -> bool {
        use sync::atomic::{AtomicUsize, Ordering};

        const GETRANDOM_UNKNOWN: usize = 0;
        const GETRANDOM_AVAILABLE: usize = 1;
        const GETRANDOM_UNAVAILABLE: usize = 2;

        static AVAILABLE: AtomicUsize = AtomicUsize::new(GETRANDOM_UNKNOWN);

        match AVAILABLE.load(Ordering::Relaxed) {
            GETRANDOM_AVAILABLE => return true,
            GETRANDOM_UNAVAILABLE => return false,
            _ => {},
        }

        let mut buf: [u8; 0] = [];
        let result = cvtu(getrandom(&mut buf));
        let available = match result {
            Ok(_) => true,
            Err(e) => e.raw_os_error() != Some(errno::ENOSYS),
        };

        AVAILABLE.store(if available {
            GETRANDOM_AVAILABLE
        } else {
            GETRANDOM_UNAVAILABLE
        }, Ordering::Relaxed);

        available
    }

    pub fn fill_bytes(v: &mut [u8]) {
        if is_getrandom_available() && getrandom_fill_bytes(v) {
            return
        }

        let mut file = File::open("/dev/urandom")
            .expect("failed to open /dev/urandom");
        file.read_exact(v).expect("failed to read /dev/urandom");
    }
}
