#![stable(feature = "steed", since = "1.0.0")]

use core::fmt;

use ctypes::c_int;
use {cmp, linux, io, memchr, str};

// Rust 1.15.0
pub mod prelude;
// Rust 1.15.0
mod buffered;
// Rust 1.15.0
mod cursor;
// Rust 1.15.0
mod error;
// Rust 1.15.0
mod impls;
// Rust 1.15.0
mod util;

const STDIN: c_int = 0;
const STDOUT: c_int = 1;
const STDERR: c_int = 2;

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::buffered::{BufReader, BufWriter, LineWriter};
#[stable(feature = "rust1", since = "1.0.0")]
pub use self::buffered::IntoInnerError;
#[stable(feature = "rust1", since = "1.0.0")]
pub use self::cursor::Cursor;
#[stable(feature = "rust1", since = "1.0.0")]
pub use self::error::{Result, Error, ErrorKind};
#[stable(feature = "rust1", since = "1.0.0")]
pub use self::util::{copy, sink, Sink, empty, Empty, repeat, Repeat};

const DEFAULT_BUF_SIZE: usize = ::sys_common::io::DEFAULT_BUF_SIZE;

#[stable(feature = "steed", since = "1.0.0")]
pub trait Read {
    #[stable(feature = "steed", since = "1.0.0")]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    #[stable(feature = "steed", since = "1.0.0")]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        read_to_end(self, buf)
    }
    #[stable(feature = "steed", since = "1.0.0")]
    fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
        append_to_string(buf, |b| read_to_end(self, b))
    }
    #[stable(feature = "read_exact", since = "1.6.0")]
    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => { let tmp = buf; buf = &mut tmp[n..]; }
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        if !buf.is_empty() {
            Err(Error::new(ErrorKind::UnexpectedEof,
                           "failed to fill whole buffer"))
        } else {
            Ok(())
        }
    }
    #[stable(feature = "steed", since = "1.0.0")]
    fn by_ref(&mut self) -> &mut Self where Self: Sized { self }
}

fn read_to_end<R: Read + ?Sized>(r: &mut R, buf: &mut Vec<u8>) -> Result<usize> {
    let start_len = buf.len();
    let mut len = start_len;
    let mut new_write_size = 16;
    let ret;
    loop {
        if len == buf.len() {
            if new_write_size < DEFAULT_BUF_SIZE {
                new_write_size *= 2;
            }
            buf.resize(len + new_write_size, 0);
        }

        match r.read(&mut buf[len..]) {
            Ok(0) => {
                ret = Ok(len - start_len);
                break;
            }
            Ok(n) => len += n,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => {
                ret = Err(e);
                break;
            }
        }
    }

    buf.truncate(len);
    ret
}

fn append_to_string<F>(buf: &mut String, f: F) -> Result<usize>
    where F: FnOnce(&mut Vec<u8>) -> Result<usize>
{
    struct Guard<'a> { s: &'a mut Vec<u8>, len: usize }
        impl<'a> Drop for Guard<'a> {
        fn drop(&mut self) {
            unsafe { self.s.set_len(self.len); }
        }
    }

    unsafe {
        let mut g = Guard { len: buf.len(), s: buf.as_mut_vec() };
        let ret = f(g.s);
        if str::from_utf8(&g.s[g.len..]).is_err() {
            ret.and_then(|_| {
                Err(Error::new(ErrorKind::InvalidData,
                               "stream did not contain valid UTF-8"))
            })
        } else {
            g.len = g.s.len();
            ret
        }
    }
}


#[stable(feature = "steed", since = "1.0.0")]
pub trait Write {
    #[stable(feature = "steed", since = "1.0.0")]
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    #[stable(feature = "steed", since = "1.0.0")]
    fn flush(&mut self) -> Result<()>;
    #[stable(feature = "steed", since = "1.0.0")]
    fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                // NOTE(steed): Deviate from std here: Construct the error
                // using `ErrorKind` instead of `Error::new`, avoiding an
                // allocation.
                Ok(0) => return Err(Error::from(ErrorKind::WriteZero)),
                Ok(n) => {
                    // NOTE(steed): Deviate from std here, in order to avoid a
                    // panic: If a `Write` implementation returns values larger
                    // than the size of the passed slice, then something is
                    // wrong with it.
                    debug_assert!(n <= buf.len());
                    buf = &buf[cmp::min(n, buf.len())..]
                },
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
    #[stable(feature = "steed", since = "1.0.0")]
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()> {
        // Create a shim which translates a Write to a fmt::Write and saves
        // off I/O errors. instead of discarding them
        struct Adaptor<'a, T: ?Sized + 'a> {
            inner: &'a mut T,
            error: Result<()>,
        }

        impl<'a, T: Write + ?Sized> fmt::Write for Adaptor<'a, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_all(s.as_bytes()) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Err(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut output = Adaptor { inner: self, error: Ok(()) };
        match fmt::write(&mut output, fmt) {
            Ok(()) => Ok(()),
            Err(..) => {
                // check if the error came from the underlying `Write` or not
                if output.error.is_err() {
                    output.error
                } else {
                    Err(Error::new(ErrorKind::Other, "formatter error"))
                }
            }
        }
    }
    #[stable(feature = "steed", since = "1.0.0")]
    fn by_ref(&mut self) -> &mut Self where Self: Sized { self }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub trait BufRead: Read {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fill_buf(&mut self) -> Result<&[u8]>;
    #[stable(feature = "rust1", since = "1.0.0")]
    fn consume(&mut self, amt: usize);
    #[stable(feature = "rust1", since = "1.0.0")]
    fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> Result<usize> {
        read_until(self, byte, buf)
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    fn read_line(&mut self, buf: &mut String) -> Result<usize> {
        // Note that we are not calling the `.read_until` method here, but
        // rather our hardcoded implementation. For more details as to why, see
        // the comments in `read_to_end`.
        append_to_string(buf, |b| read_until(self, b'\n', b))
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    fn split(self, byte: u8) -> Split<Self> where Self: Sized {
        Split { buf: self, delim: byte }
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    fn lines(self) -> Lines<Self> where Self: Sized {
        Lines { buf: self }
    }
}

fn read_until<R: BufRead + ?Sized>(r: &mut R, delim: u8, buf: &mut Vec<u8>)
                                   -> Result<usize> {
    let mut read = 0;
    loop {
        let (done, used) = {
            let available = match r.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e)
            };
            match memchr::memchr(delim, available) {
                Some(i) => {
                    buf.extend_from_slice(&available[..i + 1]);
                    (true, i + 1)
                }
                None => {
                    buf.extend_from_slice(available);
                    (false, available.len())
                }
            }
        };
        r.consume(used);
        read += used;
        if done || used == 0 {
            return Ok(read);
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub struct Split<B> {
    buf: B,
    delim: u8,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<B: BufRead> Iterator for Split<B> {
    type Item = Result<Vec<u8>>;

    fn next(&mut self) -> Option<Result<Vec<u8>>> {
        let mut buf = Vec::new();
        match self.buf.read_until(self.delim, &mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                if buf[buf.len() - 1] == self.delim {
                    buf.pop();
                }
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e))
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub struct Lines<B> {
    buf: B,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<B: BufRead> Iterator for Lines<B> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Result<String>> {
        let mut buf = String::new();
        match self.buf.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                if buf.ends_with("\n") {
                    buf.pop();
                    if buf.ends_with("\r") {
                        buf.pop();
                    }
                }
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e))
        }
    }
}


#[derive(Copy, PartialEq, Eq, Clone, Debug)]
#[stable(feature = "steed", since = "1.0.0")]
pub enum SeekFrom {
    #[stable(feature = "steed", since = "1.0.0")]
    Start(#[stable(feature = "steed", since = "1.0.0")] u64),
    #[stable(feature = "steed", since = "1.0.0")]
    End(#[stable(feature = "steed", since = "1.0.0")] i64),
    #[stable(feature = "steed", since = "1.0.0")]
    Current(#[stable(feature = "steed", since = "1.0.0")] i64),
}

#[stable(feature = "steed", since = "1.0.0")]
pub trait Seek {
    #[stable(feature = "steed", since = "1.0.0")]
    fn seek(&mut self, pos: SeekFrom) -> Result<u64>;
}

#[stable(feature = "steed", since = "1.0.0")]
pub struct Stderr {
    _0: (),
}

#[stable(feature = "steed", since = "1.0.0")]
impl Write for Stderr {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        match unsafe {
            linux::write(STDERR, buffer.as_ptr() as *const _, buffer.len())
        } {
            n if n >= 0 => Ok(n as usize),
            n => Err(Error::from_raw_os_error(-n as i32)),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[stable(feature = "steed", since = "1.0.0")]
impl fmt::Write for Stderr {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

#[stable(feature = "steed", since = "1.0.0")]
pub struct Stdin {
    _0: (),
}

#[stable(feature = "steed", since = "1.0.0")]
impl Read for Stdin {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        match unsafe {
            linux::read(STDIN, buffer.as_mut_ptr() as *mut _, buffer.len())
        } {
            n if n >= 0 => Ok(n as usize),
            n => Err(Error::from_raw_os_error(-n as i32)),
        }
    }
}

#[stable(feature = "steed", since = "1.0.0")]
pub struct Stdout {
    _0: (),
}

#[stable(feature = "steed", since = "1.0.0")]
impl Write for Stdout {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        match unsafe {
            linux::write(STDOUT, buffer.as_ptr() as *const _, buffer.len())
        } {
            n if n >= 0 => Ok(n as usize),
            n => Err(Error::from_raw_os_error(-n as i32)),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[stable(feature = "steed", since = "1.0.0")]
impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

#[stable(feature = "steed", since = "1.0.0")]
pub fn stderr() -> Stderr {
    Stderr { _0: () }
}

#[stable(feature = "steed", since = "1.0.0")]
pub fn stdin() -> Stdin {
    Stdin { _0: () }
}

#[stable(feature = "steed", since = "1.0.0")]
pub fn stdout() -> Stdout {
    Stdout { _0: () }
}

#[stable(feature = "steed", since = "1.0.0")]
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    if io::Write::write_fmt(&mut stdout(), args).is_err() {
        panic!("failed printing to stdout")
    }
}
