use ctypes::{c_int, c_uint};
use io::{Error, Read, Write};
use linux::types::umode_t;
use {linux, io};

pub struct File {
    fd: c_uint,
}

impl File {
    /// NOTE `path` must be null terminated
    pub fn create(path: &[u8]) -> io::Result<File> {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
    }

    /// NOTE `path` must be null terminated
    pub fn open(path: &[u8]) -> io::Result<File> {
        OpenOptions::new().read(true).open(path)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { linux::close(self.fd) };
    }
}

impl Read for File {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        match unsafe {
            linux::read(self.fd, buffer.as_mut_ptr() as *mut _, buffer.len())
        } {
            n if n >= 0 => Ok(n as usize),
            n => Err(Error::from_raw_os_error(-n as i32)),
        }
    }
}

impl Write for File {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        match unsafe {
            linux::write(self.fd, buffer.as_ptr() as *const _, buffer.len())
        } {
            n if n >= 0 => Ok(n as usize),
            n => Err(Error::from_raw_os_error(-n as i32)),
        }
    }
}

pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
    custom_flags: i32,
    mode: umode_t,
}

impl OpenOptions {
    pub fn new() -> Self {
        OpenOptions {
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
            custom_flags: 0,
            mode: 0o666,
        }
    }

    pub fn read(&mut self, read: bool) -> &mut Self {
        self.read = read;
        self
    }

    pub fn write(&mut self, write: bool) -> &mut Self {
        self.write = write;
        self
    }

    pub fn append(&mut self, append: bool) -> &mut Self {
        self.append = append;
        self
    }

    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate = truncate;
        self
    }

    pub fn create(&mut self, create: bool) -> &mut Self {
        self.create = create;
        self
    }

    pub fn open(&self, path: &[u8]) -> io::Result<File> {
        let flags = linux::O_CLOEXEC | self.get_access_mode()? |
                    self.get_creation_mode()? |
                    (self.custom_flags as c_int & !linux::O_ACCMODE);
        // TODO
        // fd.set_cloexec()?;

        match unsafe {
            linux::open(path.as_ptr() as *const _, flags, self.mode)
        } {
            n if n > 0 => Ok(File { fd: n as c_uint }),
            n => Err(Error::from_raw_os_error(-n as i32)),
        }
    }

    fn get_access_mode(&self) -> io::Result<c_int> {
        match (self.read, self.write, self.append) {
            (true, false, false) => Ok(linux::O_RDONLY),
            (false, true, false) => Ok(linux::O_WRONLY),
            (true, true, false) => Ok(linux::O_RDWR),
            (false, _, true) => Ok(linux::O_WRONLY | linux::O_APPEND),
            (true, _, true) => Ok(linux::O_RDWR | linux::O_APPEND),
            // FIXME error code
            (false, false, false) => Err(Error::from_raw_os_error(0)),
        }
    }

    // FIXME error code
    fn get_creation_mode(&self) -> io::Result<c_int> {
        match (self.write, self.append) {
            (true, false) => {}
            (false, false) => {
                if self.truncate || self.create || self.create_new {
                    return Err(Error::from_raw_os_error(0));
                }
            }
            (_, true) => {
                if self.truncate && !self.create_new {
                    return Err(Error::from_raw_os_error(0));
                }
            }
        }

        Ok(match (self.create, self.truncate, self.create_new) {
            (false, false, false) => 0,
            (true, false, false) => linux::O_CREAT,
            (false, true, false) => linux::O_TRUNC,
            (true, true, false) => linux::O_CREAT | linux::O_TRUNC,
            (_, _, true) => linux::O_CREAT | linux::O_EXCL,
        })
    }
}