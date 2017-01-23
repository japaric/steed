use os::unix::prelude::*;

use ctypes::c_int;
use ctypes::c_ushort;
use ffi::{CString, CStr, OsString, OsStr};
use fmt;
use io::{self, Error, SeekFrom};
use linux::types::{mode_t, stat64};
use linux;
use mem;
use path::{Path, PathBuf};
use super::{cvt, cvt_r};
use sys::errno;
use sys::ext::ffi::OsStrExt;
use sys::fd::FileDesc;
use sys::time::SystemTime;
use sys::{AsInner, FromInner};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Dir { }
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
enum dirent64 { }

pub struct File(FileDesc);

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
    pub fn size(&self) -> u64 { self.stat.st_size as u64 }
    pub fn perm(&self) -> FilePermissions {
        FilePermissions { mode: self.stat.st_mode & 0o777 }
    }
    pub fn file_type(&self) -> FileType {
        FileType { mode: self.stat.st_mode }
    }
    pub fn modified(&self) -> io::Result<SystemTime> {
        Ok(SystemTime::from(linux::timespec {
            tv_sec: self.stat.st_mtime,
            tv_nsec: self.stat.st_mtime_nsec,
        }))
    }
    pub fn accessed(&self) -> io::Result<SystemTime> {
        Ok(SystemTime::from(linux::timespec {
            tv_sec: self.stat.st_atime,
            tv_nsec: self.stat.st_atime_nsec,
        }))
    }
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
    pub fn readonly(&self) -> bool { self.mode & 0o222 == 0 }
    pub fn set_readonly(&mut self, readonly: bool) {
        if readonly {
            self.mode &= !0o222;
        } else {
            self.mode |= 0o222;
        }
    }
    pub fn mode(&self) -> u32 { self.mode as u32 }
}

impl FileType {
    pub fn is_dir(&self) -> bool { self.is(linux::S_IFDIR) }
    pub fn is_file(&self) -> bool { self.is(linux::S_IFREG) }
    pub fn is_symlink(&self) -> bool { self.is(linux::S_IFLNK) }

    pub fn is(&self, mode: mode_t) -> bool { self.mode & linux::S_IFMT == mode }
}

/*
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
*/

impl Drop for Dir {
    fn drop(&mut self) {
        unimplemented!();
    }
}

impl DirEntry {
    /*
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
    */
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
            mode: 0o666,
        }
    }

    pub fn read(&mut self, read: bool) { self.read = read; }
    pub fn write(&mut self, write: bool) { self.write = write; }
    pub fn append(&mut self, append: bool) { self.append = append; }
    pub fn truncate(&mut self, truncate: bool) { self.truncate = truncate; }
    pub fn create(&mut self, create: bool) { self.create = create; }
    pub fn create_new(&mut self, create_new: bool) { self.create_new = create_new; }

    pub fn custom_flags(&mut self, flags: i32) { self.custom_flags = flags; }
    /*pub fn mode(&mut self, mode: u32) { unimplemented!(); }*/

    fn get_access_mode(&self) -> io::Result<c_int> {
        match (self.read, self.write, self.append) {
            (true,  false, false) => Ok(linux::O_RDONLY),
            (false, true,  false) => Ok(linux::O_WRONLY),
            (true,  true,  false) => Ok(linux::O_RDWR),
            (false, _,     true)  => Ok(linux::O_WRONLY | linux::O_APPEND),
            (true,  _,     true)  => Ok(linux::O_RDWR | linux::O_APPEND),
            (false, false, false) => Err(Error::from_raw_os_error(errno::EINVAL)),
        }
    }

    fn get_creation_mode(&self) -> io::Result<c_int> {
        match (self.write, self.append) {
            (true, false) => {}
            (false, false) => {
                if self.truncate || self.create || self.create_new {
                    return Err(Error::from_raw_os_error(errno::EINVAL));
                }
            },
            (_, true) => {
                if self.truncate && !self.create_new {
                    return Err(Error::from_raw_os_error(errno::EINVAL));
                }
            },
        }

        Ok(match (self.create, self.truncate, self.create_new) {
            (false, false, false) => 0,
            (true,  false, false) => linux::O_CREAT,
            (false, true,  false) => linux::O_TRUNC,
            (true,  true,  false) => linux::O_CREAT | linux::O_TRUNC,
            (_,      _,    true)  => linux::O_CREAT | linux::O_EXCL,
       })
    }
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        let path = cstr(path)?;
        File::open_c(&path, opts)
    }

    pub fn open_c(path: &CStr, opts: &OpenOptions) -> io::Result<File> {
        let flags = linux::O_CLOEXEC |
                    linux::O_LARGEFILE |
                    opts.get_access_mode()? |
                    opts.get_creation_mode()? |
                    (opts.custom_flags as c_int & !linux::O_ACCMODE);
        let fd = FileDesc::new(cvt_r(|| unsafe {
            linux::open(path.as_ptr(), flags, opts.mode as c_ushort)
        })? as c_int);

        // Currently the standard library supports Linux 2.6.18 which did not
        // have the O_CLOEXEC flag (passed above). If we're running on an older
        // Linux kernel then the flag is just ignored by the OS, so we continue
        // to explicitly ask for a CLOEXEC fd here.
        //
        // The CLOEXEC flag, however, is supported on versions of OSX/BSD/etc
        // that we support, so we only do this on Linux currently.
        if cfg!(target_os = "linux") {
            fd.set_cloexec()?;
        }

        Ok(File(fd))
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        let mut stat: stat64 = unsafe { mem::zeroed() };
        cvt(unsafe {
            linux::fstat64(self.0.raw(), &mut stat)
        })?;
        Ok(FileAttr { stat: stat })
    }

    pub fn fsync(&self) -> io::Result<()> {
        cvt_r(|| unsafe { linux::fsync(self.0.raw()) })?;
        Ok(())
    }

    pub fn datasync(&self) -> io::Result<()> {
        cvt_r(|| unsafe { linux::fdatasync(self.0.raw()) })?;
        Ok(())
    }

    pub fn truncate(&self, size: u64) -> io::Result<()> {
        cvt_r(|| unsafe {
            linux::ftruncate64(self.0.raw(), size as i64)
        }).map(|_| ())
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }

    pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.0.read_to_end(buf)
    }

    pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        self.0.read_at(buf, offset)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        self.0.write_at(buf, offset)
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        let (whence, pos) = match pos {
            // Casting to `i64` is fine, too large values will end up as
            // negative which will cause an error in `lseek64`.
            SeekFrom::Start(off) => (linux::SEEK_SET, off as i64),
            SeekFrom::End(off) => (linux::SEEK_END, off),
            SeekFrom::Current(off) => (linux::SEEK_CUR, off),
        };
        let mut n = 0;
        cvt(unsafe { linux::_llseek(self.0.raw(), pos, &mut n, whence) })?;
        Ok(n as u64)
    }

    pub fn duplicate(&self) -> io::Result<File> {
        self.0.duplicate().map(File)
    }

    pub fn fd(&self) -> &FileDesc { &self.0 }

    pub fn into_fd(self) -> FileDesc { self.0 }
}

/*
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
*/

fn cstr(path: &Path) -> io::Result<CString> {
    Ok(CString::new(path.as_os_str().as_bytes())?)
}

/*
impl FromInner<c_int> for File {
    fn from_inner(fd: c_int) -> File {
        unimplemented!();
    }
}
*/

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn get_path(fd: c_int) -> Option<PathBuf> {
            let mut p = PathBuf::from("/proc/self/fd");
            p.push(&fd.to_string());
            readlink(&p).ok()
        }

        fn get_mode(fd: c_int) -> Option<(bool, bool)> {
            let mode = unsafe { linux::fcntl(fd, linux::F_GETFL, 0) };
            if mode == -1 {
                return None;
            }
            match mode as i32 & linux::O_ACCMODE {
                linux::O_RDONLY => Some((true, false)),
                linux::O_RDWR => Some((true, true)),
                linux::O_WRONLY => Some((false, true)),
                _ => None
            }
        }

        let fd = self.0.raw();
        let mut b = f.debug_struct("File");
        b.field("fd", &fd);
        if let Some(path) = get_path(fd) {
            b.field("path", &path);
        }
        if let Some((read, write)) = get_mode(fd) {
            b.field("read", &read).field("write", &write);
        }
        b.finish()
    }
}

/*
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
*/

pub fn readlink(p: &Path) -> io::Result<PathBuf> {
    let c_path = cstr(p)?;
    let p = c_path.as_ptr();

    let mut buf = Vec::with_capacity(256);

    loop {
        let buf_read = cvt(unsafe {
            linux::readlink(p, buf.as_mut_ptr() as *mut _, buf.capacity() as i32)
        })?;

        unsafe { buf.set_len(buf_read); }

        if buf_read != buf.capacity() {
            buf.shrink_to_fit();

            return Ok(PathBuf::from(OsString::from_vec(buf)));
        }

        // Trigger the internal buffer resizing logic of `Vec` by requiring
        // more space than the current capacity. The length is guaranteed to be
        // the same as the capacity due to the if statement above.
        buf.reserve(1);
    }
}

/*
pub fn symlink(src: &Path, dst: &Path) -> io::Result<()> {
    unimplemented!();
}

pub fn link(src: &Path, dst: &Path) -> io::Result<()> {
    unimplemented!();
}
*/

pub fn stat(p: &Path) -> io::Result<FileAttr> {
    let p = cstr(p)?;
    let mut stat: stat64 = unsafe { mem::zeroed() };
    cvt(unsafe {
        linux::stat64(p.as_ptr(), &mut stat)
    })?;
    Ok(FileAttr { stat: stat })
}

pub fn lstat(p: &Path) -> io::Result<FileAttr> {
    let p = cstr(p)?;
    let mut stat: stat64 = unsafe { mem::zeroed() };
    cvt(unsafe {
        linux::lstat64(p.as_ptr(), &mut stat)
    })?;
    Ok(FileAttr { stat: stat })
}

/*
pub fn canonicalize(p: &Path) -> io::Result<PathBuf> {
    unimplemented!();
}

pub fn copy(from: &Path, to: &Path) -> io::Result<u64> {
    unimplemented!();
}
*/
