use ffi::{CString, CStr, OsString, OsStr};
use fmt;
use io::{self, Error, ErrorKind, SeekFrom};
use mem;
use path::{Path, PathBuf};
use ptr;
use ctypes::c_int;
use sys::{AsInner, FromInner};
use sys::ext::ffi::{OsStrExt, OsStringExt};
use sys::time::SystemTime;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum FileDesc { }
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Dir { }
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum dirent64 { }
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum mode_t { }

pub struct File(FileDesc);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum stat64 { }
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReadDir { }
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DirEntry { }

#[derive(Clone)]
pub struct FileAttr {
    stat: stat64,
}

#[derive(Clone)]
pub struct OpenOptions {
    // generic
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
    // system-specific
    custom_flags: i32,
    mode: mode_t,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FilePermissions { mode: mode_t }

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileType { mode: mode_t }

pub struct DirBuilder { mode: mode_t }

impl FileAttr {
    pub fn size(&self) -> u64 { unimplemented!(); }
    pub fn perm(&self) -> FilePermissions { unimplemented!(); }
    pub fn file_type(&self) -> FileType { unimplemented!(); }
    pub fn modified(&self) -> io::Result<SystemTime> { unimplemented!(); }
    pub fn accessed(&self) -> io::Result<SystemTime> { unimplemented!(); }
    pub fn created(&self) -> io::Result<SystemTime> {
        Err(io::Error::new(io::ErrorKind::Other,
                           "creation time is not available on this platform \
                            currently"))
    }
}

impl AsInner<stat64> for FileAttr {
    fn as_inner(&self) -> &stat64 { &self.stat }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool { unimplemented!(); }
    pub fn set_readonly(&mut self, readonly: bool) { unimplemented!(); }
    pub fn mode(&self) -> u32 { unimplemented!(); }
}

impl FileType {
    pub fn is_dir(&self) -> bool { unimplemented!(); }
    pub fn is_file(&self) -> bool { unimplemented!(); }
    pub fn is_symlink(&self) -> bool { unimplemented!(); }
    pub fn is(&self, mode: mode_t) -> bool { unimplemented!(); }
}

impl FromInner<u32> for FilePermissions {
    fn from_inner(mode: u32) -> FilePermissions {
        unimplemented!();
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> { unimplemented!(); }
}

impl Drop for Dir {
    fn drop(&mut self) {
        unimplemented!();
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        unimplemented!();
    }

    pub fn file_name(&self) -> OsString {
        OsStr::from_bytes(self.name_bytes()).to_os_string()
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        unimplemented!();
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        unimplemented!();
    }

    pub fn ino(&self) -> u64 {
        unimplemented!();
    }

    fn name_bytes(&self) -> &[u8] {
        unimplemented!();
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            // generic
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
            // system-specific
            custom_flags: 0,
            mode: unimplemented!(),
        }
    }

    pub fn read(&mut self, read: bool) { self.read = read; }
    pub fn write(&mut self, write: bool) { self.write = write; }
    pub fn append(&mut self, append: bool) { self.append = append; }
    pub fn truncate(&mut self, truncate: bool) { self.truncate = truncate; }
    pub fn create(&mut self, create: bool) { self.create = create; }
    pub fn create_new(&mut self, create_new: bool) { self.create_new = create_new; }

    pub fn custom_flags(&mut self, flags: i32) { self.custom_flags = flags; }
    pub fn mode(&mut self, mode: u32) { unimplemented!(); }

    fn get_access_mode(&self) -> io::Result<c_int> {
        unimplemented!();
    }

    fn get_creation_mode(&self) -> io::Result<c_int> {
        unimplemented!();
    }
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        unimplemented!();
    }

    pub fn open_c(path: &CStr, opts: &OpenOptions) -> io::Result<File> {
        unimplemented!();
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        unimplemented!();
    }

    pub fn fsync(&self) -> io::Result<()> {
        unimplemented!();
    }

    pub fn datasync(&self) -> io::Result<()> {
        unimplemented!();
    }

    pub fn truncate(&self, size: u64) -> io::Result<()> {
        unimplemented!();
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!();
    }

    pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        unimplemented!();
    }

    pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        unimplemented!();
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        unimplemented!();
    }

    pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        unimplemented!();
    }

    pub fn flush(&self) -> io::Result<()> {
        unimplemented!();
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        unimplemented!();
    }

    pub fn duplicate(&self) -> io::Result<File> {
        unimplemented!();
    }

    pub fn fd(&self) -> &FileDesc { &self.0 }

    pub fn into_fd(self) -> FileDesc { self.0 }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        unimplemented!();
    }

    pub fn mkdir(&self, p: &Path) -> io::Result<()> {
        unimplemented!();
    }

    pub fn set_mode(&mut self, mode: u32) {
        unimplemented!();
    }
}

fn cstr(path: &Path) -> io::Result<CString> {
    Ok(CString::new(path.as_os_str().as_bytes())?)
}

impl FromInner<c_int> for File {
    fn from_inner(fd: c_int) -> File {
        unimplemented!();
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

pub fn readdir(p: &Path) -> io::Result<ReadDir> {
    unimplemented!();
}

pub fn unlink(p: &Path) -> io::Result<()> {
    unimplemented!();
}

pub fn rename(old: &Path, new: &Path) -> io::Result<()> {
    unimplemented!();
}

pub fn set_perm(p: &Path, perm: FilePermissions) -> io::Result<()> {
    unimplemented!();
}

pub fn rmdir(p: &Path) -> io::Result<()> {
    unimplemented!();
}

pub fn remove_dir_all(path: &Path) -> io::Result<()> {
    unimplemented!();
}

fn remove_dir_all_recursive(path: &Path) -> io::Result<()> {
    unimplemented!();
}

pub fn readlink(p: &Path) -> io::Result<PathBuf> {
    unimplemented!();
}

pub fn symlink(src: &Path, dst: &Path) -> io::Result<()> {
    unimplemented!();
}

pub fn link(src: &Path, dst: &Path) -> io::Result<()> {
    unimplemented!();
}

pub fn stat(p: &Path) -> io::Result<FileAttr> {
    unimplemented!();
}

pub fn lstat(p: &Path) -> io::Result<FileAttr> {
    unimplemented!();
}

pub fn canonicalize(p: &Path) -> io::Result<PathBuf> {
    unimplemented!();
}

pub fn copy(from: &Path, to: &Path) -> io::Result<u64> {
    unimplemented!();
}
