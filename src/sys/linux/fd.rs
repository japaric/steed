// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![unstable(reason = "not public", issue = "0", feature = "fd")]

use ctypes::{c_int, c_char};
use io::{self, Read};
use linux;
use mem;
use sys::cvt;
use sys_common::AsInner;
use sys_common::io::read_to_end_uninitialized;

pub struct FileDesc {
    fd: c_int,
}

impl FileDesc {
    pub fn new(fd: c_int) -> FileDesc {
        FileDesc { fd: fd }
    }

    pub fn raw(&self) -> c_int { self.fd }

    /// Extracts the actual filedescriptor without closing it.
    pub fn into_raw(self) -> c_int {
        let fd = self.fd;
        mem::forget(self);
        fd
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        Ok(cvt(unsafe {
            linux::read(self.fd, buf.as_mut_ptr() as *mut c_char, buf.len())
        })?)
    }

    pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut me = self;
        (&mut me).read_to_end(buf)
    }

    /*
    pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        unimplemented!();
    }
    */

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        Ok(cvt(unsafe {
            linux::write(self.fd, buf.as_ptr() as *const c_char, buf.len())
        })?)
    }

    /*
    pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        unimplemented!();
    }
    */

    pub fn set_cloexec(&self) -> io::Result<()> {
        unsafe {
            cvt(linux::ioctl(self.fd, linux::FIOCLEX, 0))?;
        }
        Ok(())
    }

    /*
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        unimplemented!();
    }

    pub fn duplicate(&self) -> io::Result<FileDesc> {
        unimplemented!();
    }
    */
}

impl<'a> Read for &'a FileDesc {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (**self).read(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        unsafe { read_to_end_uninitialized(self, buf) }
    }
}

impl AsInner<c_int> for FileDesc {
    fn as_inner(&self) -> &c_int { &self.fd }
}

impl Drop for FileDesc {
    fn drop(&mut self) {
        // Note that errors are ignored when closing a file descriptor. The
        // reason for this is that if an error occurs we don't actually know if
        // the file descriptor was closed or not, and if we retried (for
        // something like EINTR), we might close another valid file descriptor
        // (opened after we closed ours.
        let _ = cvt(unsafe { linux::close(self.fd) });
    }
}
