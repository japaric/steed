#![allow(non_camel_case_types)]
#![allow(unused)]

pub enum read2 { }
use sync::atomic::{AtomicBool, Ordering};
use cmp;
use io;
use linux::errno;
use linux;
use mem;
use ptr;
use sys::cvt;
use sys::fd::FileDesc;

////////////////////////////////////////////////////////////////////////////////
// Anonymous pipes
////////////////////////////////////////////////////////////////////////////////

pub struct AnonPipe(FileDesc);

pub fn anon_pipe() -> io::Result<(AnonPipe, AnonPipe)> {
    let mut fds = [0; 2];

    // Unfortunately the only known way right now to create atomically set the
    // CLOEXEC flag is to use the `pipe2` syscall on Linux. This was added in
    // 2.6.27, however, and because we support 2.6.18 we must detect this
    // support dynamically.
    static TRY_PIPE2: AtomicBool = AtomicBool::new(true);
    if TRY_PIPE2.load(Ordering::Relaxed) {
        match cvt(unsafe { linux::pipe2(fds.as_mut_ptr(), linux::O_CLOEXEC) }) {
            Err(ref e) if e.raw_os_error() == Some(errno::ENOSYS) => {
                TRY_PIPE2.store(false, Ordering::Relaxed);
                // Fall through.
            }
            res => {
                res?;
                return Ok((AnonPipe(FileDesc::new(fds[0])),
                           AnonPipe(FileDesc::new(fds[1]))));
            }
        }
    }
    cvt(unsafe { linux::pipe(fds.as_mut_ptr()) })?;
    let fd0 = FileDesc::new(fds[0]);
    let fd1 = FileDesc::new(fds[1]);
    fd0.set_cloexec()?;
    fd1.set_cloexec()?;
    Ok((AnonPipe(fd0), AnonPipe(fd1)))
}

impl AnonPipe {
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }

    pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.0.read_to_end(buf)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    pub fn fd(&self) -> &FileDesc { &self.0 }
    pub fn into_fd(self) -> FileDesc { self.0 }
}
