// Copyright 2013-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use self::imp::OsRng;

use mem;

fn next_u32(fill_buf: &mut FnMut(&mut [u8])) -> u32 {
    let mut buf: [u8; 4] = [0; 4];
    fill_buf(&mut buf);
    unsafe { mem::transmute::<[u8; 4], u32>(buf) }
}

fn next_u64(fill_buf: &mut FnMut(&mut [u8])) -> u64 {
    let mut buf: [u8; 8] = [0; 8];
    fill_buf(&mut buf);
    unsafe { mem::transmute::<[u8; 8], u64>(buf) }
}

mod imp {
    use self::OsRngInner::*;
    use super::{next_u32, next_u64};

    use ctypes::{c_char, ssize_t};
    use fs::File;
    use io::{self, ErrorKind};
    use linux::{self, errno};
    use rand::Rng;
    use rand::reader::ReaderRng;
    use sys::cvtu;

    fn getrandom(buf: &mut [u8]) -> ssize_t {
        unsafe {
            linux::getrandom(buf.as_mut_ptr() as *mut c_char,
                             buf.len(),
                             linux::GRND_NONBLOCK)
        }
    }

    fn getrandom_fill_bytes(v: &mut [u8]) {
        let mut read = 0;
        while read < v.len() {
            match cvtu(getrandom(&mut v[read..])) {
                Err(e) => {
                    let kind = e.kind();
                    if kind == ErrorKind::Interrupted {
                        continue;
                    } else if kind == ErrorKind::WouldBlock {
                        // if getrandom() returns EAGAIN it would have blocked
                        // because the non-blocking pool (urandom) has not
                        // initialized in the kernel yet due to a lack of entropy
                        // the fallback we do here is to avoid blocking applications
                        // which could depend on this call without ever knowing
                        // they do and don't have a work around. The PRNG of
                        // /dev/urandom will still be used but not over a completely
                        // full entropy pool
                        let reader = File::open("/dev/urandom").expect("Unable to open /dev/urandom");
                        let mut reader_rng = ReaderRng::new(reader);
                        reader_rng.fill_bytes(&mut v[read..]);
                        read += v.len();
                    } else {
                        panic!("unexpected getrandom error: {}", e);
                    }
                }
                Ok(result) => read += result,
            }
        }
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

    pub struct OsRng {
        inner: OsRngInner,
    }

    enum OsRngInner {
        OsGetrandomRng,
        OsReaderRng(ReaderRng<File>),
    }

    impl OsRng {
        /// Create a new `OsRng`.
        pub fn new() -> io::Result<OsRng> {
            if is_getrandom_available() {
                return Ok(OsRng { inner: OsGetrandomRng });
            }

            let reader = File::open("/dev/urandom")?;
            let reader_rng = ReaderRng::new(reader);

            Ok(OsRng { inner: OsReaderRng(reader_rng) })
        }
    }

    impl Rng for OsRng {
        fn next_u32(&mut self) -> u32 {
            match self.inner {
                OsGetrandomRng => next_u32(&mut getrandom_fill_bytes),
                OsReaderRng(ref mut rng) => rng.next_u32(),
            }
        }
        fn next_u64(&mut self) -> u64 {
            match self.inner {
                OsGetrandomRng => next_u64(&mut getrandom_fill_bytes),
                OsReaderRng(ref mut rng) => rng.next_u64(),
            }
        }
        fn fill_bytes(&mut self, v: &mut [u8]) {
            match self.inner {
                OsGetrandomRng => getrandom_fill_bytes(v),
                OsReaderRng(ref mut rng) => rng.fill_bytes(v)
            }
        }
    }
}
