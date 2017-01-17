#![stable(feature = "steed", since = "1.0.0")]

use core::fmt;

use ctypes::c_uint;
use {linux, io};

// Rust 1.14.0
mod error;

const STDIN: c_uint = 0;
const STDOUT: c_uint = 1;
const STDERR: c_uint = 2;

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::error::{Result, Error, ErrorKind};

#[stable(feature = "steed", since = "1.0.0")]
pub trait Read {
    #[stable(feature = "steed", since = "1.0.0")]
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize>;
}

#[stable(feature = "steed", since = "1.0.0")]
pub trait Write {
    #[stable(feature = "steed", since = "1.0.0")]
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize>;

    #[stable(feature = "steed", since = "1.0.0")]
    fn write_all(&mut self, mut buffer: &[u8]) -> io::Result<()> {
        if buffer.len() == 0 {
            return Ok(());
        }

        loop {
            let n = self.write(buffer)?;

            if n < buffer.len() {
                buffer = &buffer[n..];
            } else {
                return Ok(());
            }
        }
    }
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
    use core::fmt::Write;

    if stdout().write_fmt(args).is_err() {
        panic!("failed printing to stdout")
    }
}
