#![stable(feature = "steed", since = "1.0.0")]

use fmt;
use io::{self, Error, Read, Write};
use libc::{self, c_int};
use str;

const STDIN: c_int = 0;
const STDOUT: c_int = 1;
const STDERR: c_int = 2;

#[stable(feature = "steed", since = "1.0.0")]
pub struct Stderr {
    _0: (),
}

#[stable(feature = "steed", since = "1.0.0")]
impl Write for Stderr {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        match unsafe {
            libc::write(STDERR, buffer.as_ptr() as *const _, buffer.len())
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
            libc::read(STDIN, buffer.as_mut_ptr() as *mut _, buffer.len())
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
            libc::write(STDOUT, buffer.as_ptr() as *const _, buffer.len())
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
