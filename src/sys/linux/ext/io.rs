#![stable(feature = "steed", since = "1.0.0")]

use fs;
use os::raw;
use sys;
use sys_common::{self, AsInner, FromInner, IntoInner};

#[stable(feature = "steed", since = "1.0.0")]
pub type RawFd = raw::c_int;

#[stable(feature = "steed", since = "1.0.0")]
pub trait AsRawFd {
    #[stable(feature = "steed", since = "1.0.0")]
    fn as_raw_fd(&self) -> RawFd;
}

#[stable(feature = "from_raw_os", since = "1.1.0")]
pub trait FromRawFd {
    #[stable(feature = "from_raw_os", since = "1.1.0")]
    unsafe fn from_raw_fd(fd: RawFd) -> Self;
}

#[stable(feature = "into_raw_os", since = "1.4.0")]
pub trait IntoRawFd {
    #[stable(feature = "into_raw_os", since = "1.4.0")]
    fn into_raw_fd(self) -> RawFd;
}

#[stable(feature = "steed", since = "1.0.0")]
impl AsRawFd for fs::File {
    fn as_raw_fd(&self) -> RawFd {
        self.as_inner().fd().raw()
    }
}
#[stable(feature = "from_raw_os", since = "1.1.0")]
impl FromRawFd for fs::File {
    unsafe fn from_raw_fd(fd: RawFd) -> fs::File {
        fs::File::from_inner(sys::fs::File::from_inner(fd))
    }
}
#[stable(feature = "into_raw_os", since = "1.4.0")]
impl IntoRawFd for fs::File {
    fn into_raw_fd(self) -> RawFd {
        self.into_inner().into_fd().into_raw()
    }
}
