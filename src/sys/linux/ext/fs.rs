#![stable(feature = "steed", since = "1.0.0")]

use fs;
use io;
use sys::AsInner;

#[unstable(feature = "file_offset", issue = "35918")]
pub trait FileExt {
    #[unstable(feature = "file_offset", issue = "35918")]
    fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize>;
    #[unstable(feature = "file_offset", issue = "35918")]
    fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize>;
}

#[unstable(feature = "file_offset", issue = "35918")]
impl FileExt for fs::File {
    fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        self.as_inner().read_at(buf, offset)
    }
    fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        self.as_inner().write_at(buf, offset)
    }
}
