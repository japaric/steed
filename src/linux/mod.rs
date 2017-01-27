#[cfg(target_arch = "aarch64")]
#[path = "aarch64.rs"]
mod arch;

#[cfg(target_arch = "arm")]
#[path = "arm.rs"]
mod arch;

#[cfg(target_arch = "mips")]
#[path = "mips.rs"]
mod arch;

#[cfg(target_arch = "mips64")]
#[path = "mips64.rs"]
mod arch;

#[cfg(target_arch = "powerpc")]
#[path = "powerpc.rs"]
mod arch;

#[cfg(target_arch = "powerpc64")]
#[path = "powerpc64.rs"]
mod arch;

#[cfg(target_arch = "sparc64")]
#[path = "sparc64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

mod libc;
mod types;

use core::intrinsics;
use ctypes::*;
use ptr;

pub use self::arch::*;
pub use self::libc::*;
pub use self::types::*;

// include/uapi/linux/fcntl.h
pub const AT_FDCWD: c_int = -100;
pub const AT_REMOVEDIR: c_int = 0x200;
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
pub const F_DUPFD_CLOEXEC: c_uint = F_LINUX_SPECIFIC_BASE + 6;

// include/uapi/asm-generic/fcntl.h
pub const F_DUPFD: c_uint = 0;
pub const F_GETFL: c_uint = 3;
pub const F_LINUX_SPECIFIC_BASE: c_uint = 1024;
pub const O_ACCMODE: c_int = 0o00000003;
pub const O_RDONLY: c_int = 0o00000000;
pub const O_RDWR: c_int = 0o00000002;
pub const O_WRONLY: c_int = 0o00000001;

// include/uapi/linux/stat.h
pub const S_IFMT: c_uint = 0o00170000;
pub const S_IFSOCK: c_uint = 0o0140000;
pub const S_IFLNK: c_uint = 0o0120000;
pub const S_IFREG: c_uint = 0o0100000;
pub const S_IFBLK: c_uint = 0o0060000;
pub const S_IFDIR: c_uint = 0o0040000;
pub const S_IFCHR: c_uint = 0o0020000;
pub const S_IFIFO: c_uint = 0o0010000;

// include/uapi/linux/time.h
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_REALTIME: clockid_t = 0;

// include/uapi/linux/fs.h
pub const SEEK_SET: c_uint = 0;
pub const SEEK_CUR: c_uint = 1;
pub const SEEK_END: c_uint = 2;

// include/linux/fs.h
pub const DT_FIFO: c_uchar = 1;
pub const DT_CHR: c_uchar = 2;
pub const DT_DIR: c_uchar = 4;
pub const DT_BLK: c_uchar = 6;
pub const DT_REG: c_uchar = 8;
pub const DT_LNK: c_uchar = 10;
pub const DT_SOCK: c_uchar = 12;

// include/uapi/linux/random.h
pub const GRND_NONBLOCK: c_uint = 0x0001;

// include/uapi/linux/in.h
pub const IPPROTO_IP: c_int = 0;
pub const IPPROTO_TCP: c_int = 6;
pub const IPPROTO_IPV6: c_int = 41;

// include/uapi/linux/in.h
pub const IP_TTL: c_int = 2;

// include/uapi/linux/in.h
pub const IP_MULTICAST_TTL: c_int = 33;
pub const IP_MULTICAST_LOOP: c_int = 34;
pub const IP_ADD_MEMBERSHIP: c_int = 35;
pub const IP_DROP_MEMBERSHIP: c_int = 36;

// include/uapi/linux/in6.h
pub const IPV6_V6ONLY: c_int = 26;
pub const IPV6_MULTICAST_LOOP: c_int = 19;

// include/uapi/linux/in6.h
pub const IPV6_ADD_MEMBERSHIP: c_int = 20;
pub const IPV6_DROP_MEMBERSHIP: c_int = 21;

// include/linux/socket.h
pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;

// include/linux/net.h
pub const SOCK_CLOEXEC: c_int = O_CLOEXEC;

// include/linux/net.h
pub const SHUT_RD: c_int = 0;
pub const SHUT_WR: c_int = 1;
pub const SHUT_RDWR: c_int = 2;

// include/uapi/linux/tcp.h
pub const TCP_NODELAY: c_int = 1;

// include/linux/socket.h
pub const MSG_NOSIGNAL: c_int = 0x4000;

// kernel/time/posix-timers.c
#[inline(always)]
pub unsafe fn clock_gettime(which_clock: clockid_t,
                            tp: *mut timespec)
                            -> c_int {
    syscall!(CLOCK_GETTIME, which_clock, tp) as isize as c_int
}

// fs/open.c
#[inline(always)]
pub unsafe fn close(fd: c_int) -> ssize_t {
    syscall!(CLOSE, fd) as ssize_t
}

// kernel/exit.c
#[inline(always)]
pub unsafe fn exit_group(code: c_int) -> ! {
    syscall!(EXIT_GROUP, code);

    intrinsics::unreachable()
}

// fs/open.c
#[inline(always)]
pub unsafe fn open(filename: *const c_char,
                   flags: c_int,
                   mode: umode_t)
                   -> ssize_t {
    syscall!(OPENAT, AT_FDCWD, filename, flags, mode) as ssize_t
}

// fs/open.c
#[inline(always)]
pub unsafe fn chmod(filename: *const c_char, mode: umode_t) -> ssize_t {
    syscall!(FCHMODAT, AT_FDCWD, filename, mode, 0) as ssize_t
}

// fs/read_write.c
#[inline(always)]
pub unsafe fn read(fd: c_int, buffer: *mut c_char, count: size_t) -> ssize_t {
    syscall!(READ, fd, buffer, count) as ssize_t
}

// fs/read_write.c
#[inline(always)]
pub unsafe fn write(fd: c_int,
                    buffer: *const c_char,
                    count: size_t)
                    -> ssize_t {
    syscall!(WRITE, fd, buffer, count) as ssize_t
}

#[cfg(all(target_endian = "big", target_pointer_width = "32"))]
fn high(v: loff_t) -> i32 {
    (v >> 32) as i32
}
#[cfg(all(target_endian = "big", target_pointer_width = "32"))]
fn low(v: loff_t) -> i32 {
    (v & 0xffff_ffff) as i32
}

#[cfg(all(target_endian = "little", target_pointer_width = "32"))]
fn high(v: loff_t) -> i32 {
    (v & 0xffff_ffff) as i32
}
#[cfg(all(target_endian = "little", target_pointer_width = "32"))]
fn low(v: loff_t) -> i32 {
    (v >> 32) as i32
}

// fs/read_write.c
#[inline(always)]
pub unsafe fn pread64(fd: c_int,
                      buffer: *mut c_char,
                      count: size_t,
                      pos: loff_t)
                      -> ssize_t {
    #[cfg(all(target_pointer_width = "32", not(target_arch = "x86")))]
    #[inline(always)]
    unsafe fn pread64(fd: c_int,
                      buffer: *const c_char,
                      count: size_t,
                      pos: loff_t)
                      -> ssize_t {
        syscall!(PREAD64, fd, buffer, count, 0, high(pos), low(pos)) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn pread64(fd: c_int,
                      buffer: *const c_char,
                      count: size_t,
                      pos: loff_t)
                      -> ssize_t {
        syscall!(PREAD64, fd, buffer, count, high(pos), low(pos)) as ssize_t
    }
    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    unsafe fn pread64(fd: c_int,
                      buffer: *const c_char,
                      count: size_t,
                      pos: loff_t)
                      -> ssize_t {
        syscall!(PREAD64, fd, buffer, count, pos) as ssize_t
    }
    pread64(fd, buffer, count, pos)
}

// fs/read_write.c
#[inline(always)]
pub unsafe fn pwrite64(fd: c_int,
                       buffer: *const c_char,
                       count: size_t,
                       pos: loff_t)
                       -> ssize_t {
    #[cfg(all(target_pointer_width = "32", not(target_arch = "x86")))]
    #[inline(always)]
    unsafe fn pwrite64(fd: c_int,
                       buffer: *const c_char,
                       count: size_t,
                       pos: loff_t)
                       -> ssize_t {
        syscall!(PWRITE64, fd, buffer, count, 0, high(pos), low(pos)) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn pwrite64(fd: c_int,
                       buffer: *const c_char,
                       count: size_t,
                       pos: loff_t)
                       -> ssize_t {
        syscall!(PWRITE64, fd, buffer, count, high(pos), low(pos)) as ssize_t
    }
    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    unsafe fn pwrite64(fd: c_int,
                       buffer: *const c_char,
                       count: size_t,
                       pos: loff_t)
                       -> ssize_t {
        syscall!(PWRITE64, fd, buffer, count, pos) as ssize_t
    }
    pwrite64(fd, buffer, count, pos)
}

// fs/read_write.c
#[inline(always)]
pub unsafe fn _llseek(fd: c_int,
                      offset: loff_t,
                      result: *mut loff_t,
                      whence: c_uint)
                      -> ssize_t {
    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    unsafe fn _llseek(fd: c_int,
                      offset: loff_t,
                      result: *mut loff_t,
                      whence: c_uint)
                      -> ssize_t {
        syscall!(_LLSEEK,
                 fd,
                 offset >> 32,
                 offset & 0xffff_ffff,
                 result,
                 whence) as ssize_t
    }
    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    unsafe fn _llseek(fd: c_int,
                      offset: loff_t,
                      result: *mut loff_t,
                      whence: c_uint)
                      -> ssize_t {
        let res = syscall!(LSEEK, fd, offset, whence) as ssize_t;
        *result = res as i64;
        res
    }
    _llseek(fd, offset, result, whence)
}

// fs/open.c
#[inline(always)]
pub unsafe fn ftruncate64(fd: c_int, length: loff_t) -> ssize_t {
    #[cfg(all(target_pointer_width = "32", not(target_arch = "x86")))]
    #[inline(always)]
    unsafe fn ftruncate64(fd: c_int, length: loff_t) -> ssize_t {
        syscall!(FTRUNCATE64, fd, 0, high(length), low(length)) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn ftruncate64(fd: c_int, length: loff_t) -> ssize_t {
        syscall!(FTRUNCATE64, fd, high(length), low(length)) as ssize_t
    }
    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    unsafe fn ftruncate64(fd: c_int, length: loff_t) -> ssize_t {
        syscall!(FTRUNCATE, fd, length) as ssize_t
    }
    ftruncate64(fd, length)
}

// fs/ioctl.c
#[inline(always)]
pub unsafe fn ioctl(fd: c_int, cmd: c_uint, arg: c_ulong) -> ssize_t {
    syscall!(IOCTL, fd, cmd, arg) as ssize_t
}

// fs/sync.c
#[inline(always)]
pub unsafe fn fsync(fd: c_int) -> ssize_t {
    syscall!(FSYNC, fd) as ssize_t
}

// fs/sync.c
#[inline(always)]
pub unsafe fn fdatasync(fd: c_int) -> ssize_t {
    syscall!(FDATASYNC, fd) as ssize_t
}

// fs/stat.c
#[inline(always)]
pub unsafe fn fstat64(fd: c_int, statbuf: *mut stat64) -> ssize_t {
    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    unsafe fn fstat64(fd: c_int, statbuf: *mut stat64) -> ssize_t {
        syscall!(FSTAT64, fd, statbuf) as ssize_t
    }
    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    unsafe fn fstat64(fd: c_int, statbuf: *mut stat64) -> ssize_t {
        syscall!(FSTAT, fd, statbuf) as ssize_t
    }
    fstat64(fd, statbuf)
}

// fs/stat.c
#[inline(always)]
pub unsafe fn stat64(filename: *const c_char, statbuf: *mut stat64) -> ssize_t {
    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    unsafe fn stat64(filename: *const c_char, statbuf: *mut stat64) -> ssize_t {
        syscall!(FSTATAT64, AT_FDCWD, filename, statbuf, 0) as ssize_t
    }
    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    unsafe fn stat64(filename: *const c_char, statbuf: *mut stat64) -> ssize_t {
        syscall!(NEWFSTATAT, AT_FDCWD, filename, statbuf, 0) as ssize_t
    }
    stat64(filename, statbuf)
}

// fs/stat.c
#[inline(always)]
pub unsafe fn lstat64(filename: *const c_char,
                      statbuf: *mut stat64)
                      -> ssize_t {
    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    unsafe fn lstat64(filename: *const c_char,
                      statbuf: *mut stat64)
                      -> ssize_t {
        syscall!(FSTATAT64,
                 AT_FDCWD,
                 filename,
                 statbuf,
                 AT_SYMLINK_NOFOLLOW) as ssize_t
    }
    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    unsafe fn lstat64(filename: *const c_char,
                      statbuf: *mut stat64)
                      -> ssize_t {
        syscall!(NEWFSTATAT,
                 AT_FDCWD,
                 filename,
                 statbuf,
                 AT_SYMLINK_NOFOLLOW) as ssize_t
    }
    lstat64(filename, statbuf)
}

// fs/stat.c
#[inline(always)]
pub unsafe fn readlink(path: *const c_char,
                       buf: *mut c_char,
                       bufsiz: c_int)
                       -> ssize_t {
    syscall!(READLINKAT, AT_FDCWD, path, buf, bufsiz) as ssize_t
}

// fs/fcntl.c
#[inline(always)]
pub unsafe fn fcntl(fd: c_int, cmd: c_uint, arg: c_ulong) -> ssize_t {
    syscall!(FCNTL, fd, cmd, arg) as ssize_t
}

// fs/namei.c
#[inline(always)]
pub unsafe fn rename(oldname: *const c_char,
                     newname: *const c_char)
                     -> ssize_t {
    syscall!(RENAMEAT, AT_FDCWD, oldname, AT_FDCWD, newname) as ssize_t
}

// fs/namei.c
#[inline(always)]
pub unsafe fn unlink(pathname: *const c_char) -> ssize_t {
    syscall!(UNLINKAT, AT_FDCWD, pathname, 0) as ssize_t
}

// fs/namei.c
#[inline(always)]
pub unsafe fn rmdir(pathname: *const c_char) -> ssize_t {
    syscall!(UNLINKAT, AT_FDCWD, pathname, AT_REMOVEDIR) as ssize_t
}

// fs/namei.c
#[inline(always)]
pub unsafe fn link(oldname: *const c_char, newname: *const c_char) -> ssize_t {
    syscall!(LINKAT, AT_FDCWD, oldname, AT_FDCWD, newname, 0) as ssize_t
}

// fs/namei.c
#[inline(always)]
pub unsafe fn symlink(oldname: *const c_char,
                      newname: *const c_char)
                      -> ssize_t {
    syscall!(SYMLINKAT, oldname, AT_FDCWD, newname) as ssize_t
}

// fs/namei.c
#[inline(always)]
pub unsafe fn mkdir(pathname: *const c_char, mode: umode_t) -> ssize_t {
    syscall!(MKDIRAT, AT_FDCWD, pathname, mode) as ssize_t
}

// fs/readdir.c
#[inline(always)]
pub unsafe fn getdents64(fd: c_int,
                         dirent: *mut linux_dirent64,
                         count: c_uint)
                         -> ssize_t {
    syscall!(GETDENTS64, fd, dirent, count) as ssize_t
}

// kernel/fork.c
#[inline(always)]
pub unsafe fn clone(clone_flags: c_ulong,
                    newsp: c_ulong,
                    parent_tidptr: *mut c_int,
                    tls: c_ulong,
                    child_tidptr: *mut c_int)
                    -> ssize_t {
    #[cfg(any(target_arch = "aarch64",
              target_arch = "arm",
              target_arch = "mips",
              target_arch = "mips64",
              target_arch = "powerpc",
              target_arch = "powerpc64",
              target_arch = "x86"))]
    #[inline(always)]
    unsafe fn clone(clone_flags: c_ulong,
                    newsp: c_ulong,
                    parent_tidptr: *mut c_int,
                    tls: c_ulong,
                    child_tidptr: *mut c_int)
                    -> ssize_t {
        syscall!(CLONE,
                 clone_flags,
                 newsp,
                 parent_tidptr,
                 tls,
                 child_tidptr) as ssize_t
    }
    #[cfg(any(target_arch = "x86_64"))]
    #[inline(always)]
    unsafe fn clone(clone_flags: c_ulong,
                    newsp: c_ulong,
                    parent_tidptr: *mut c_int,
                    tls: c_ulong,
                    child_tidptr: *mut c_int)
                    -> ssize_t {
        syscall!(CLONE,
                 clone_flags,
                 newsp,
                 parent_tidptr,
                 child_tidptr,
                 tls) as ssize_t
    }
    clone(clone_flags, newsp, parent_tidptr, tls, child_tidptr)
}

// kernel/fork.c
#[inline(always)]
pub unsafe fn fork() -> ssize_t {
    clone(SIGCHLD, 0, ptr::null_mut(), 0, ptr::null_mut())
}

// fs/exec.c
#[inline(always)]
pub unsafe fn execve(filename: *const c_char,
                     argv: *const *const c_char,
                     envp: *const *const c_char)
                     -> ssize_t {
    syscall!(EXECVE, filename, argv, envp) as ssize_t
}

// fs/pipe.c
#[inline(always)]
pub unsafe fn pipe2(filedes: *mut c_int, flags: c_int) -> ssize_t {
    syscall!(PIPE2, filedes, flags) as ssize_t
}

// fs/pipe.c
#[inline(always)]
pub unsafe fn pipe(filedes: *mut c_int) -> ssize_t {
    #[inline(always)]
    #[cfg(not(target_arch = "aarch64"))]
    unsafe fn pipe(filedes: *mut c_int) -> ssize_t {
        syscall!(PIPE, filedes) as ssize_t
    }
    #[inline(always)]
    #[cfg(target_arch = "aarch64")]
    unsafe fn pipe(filedes: *mut c_int) -> ssize_t {
        pipe2(filedes, 0)
    }
    pipe(filedes)
}

// kernel/exit.c
#[inline(always)]
pub unsafe fn wait4(upid: pid_t,
                    stat_addr: *mut c_int,
                    options: c_int,
                    ru: *mut rusage)
                    -> ssize_t {
    syscall!(WAIT4, upid, stat_addr, options, ru) as ssize_t
}

// drivers/char/random.c
pub unsafe fn getrandom(buf: *mut c_char, count: size_t, flags: c_uint) -> ssize_t {
    syscall!(GETRANDOM, buf, count, flags) as ssize_t
}

// net/socket.c
#[inline(always)]
pub unsafe fn socket(family: c_int, typ: c_int, protocol: c_int) -> ssize_t {
    #[cfg(not(target_arch = "x86"))]
    #[inline(always)]
    unsafe fn socket(family: c_int, typ: c_int, protocol: c_int) -> ssize_t {
        syscall!(SOCKET, family, typ, protocol) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn socket(family: c_int, typ: c_int, protocol: c_int) -> ssize_t {
        syscall!(SOCKETCALL,
                 SYS_SOCKET,
                 &[family as c_long,
                   typ,
                   protocol,
                   0,
                   0,
                   0] as *const _) as ssize_t
    }
    socket(family, typ, protocol)
}

// net/socket.c
#[inline(always)]
pub unsafe fn socketpair(family: c_int,
                         typ: c_int,
                         protocol: c_int,
                         fds: *mut c_int)
                         -> ssize_t {
    #[cfg(not(target_arch = "x86"))]
    #[inline(always)]
    unsafe fn socketpair(family: c_int,
                         typ: c_int,
                         protocol: c_int,
                         fds: *mut c_int)
                         -> ssize_t {
        syscall!(SOCKETPAIR, family, typ, protocol, fds) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn socketpair(family: c_int,
                         typ: c_int,
                         protocol: c_int,
                         fds: *mut c_int)
                         -> ssize_t {
        syscall!(SOCKETCALL,
                 SYS_SOCKETPAIR,
                 &[family as c_long,
                   typ,
                   protocol,
                   fds as c_long,
                   0,
                   0] as *const _) as ssize_t
    }
    socketpair(family, typ, protocol, fds)
}

// net/socket.c
#[inline(always)]
pub unsafe fn accept(fd: c_int,
                     addr: *mut sockaddr,
                     addrlen: *mut socklen_t)
                     -> ssize_t {
    #[cfg(not(target_arch = "x86"))]
    #[inline(always)]
    unsafe fn accept(fd: c_int,
                     addr: *mut sockaddr,
                     addrlen: *mut socklen_t)
                     -> ssize_t {
        syscall!(ACCEPT, fd, addr, addrlen) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn accept(fd: c_int,
                     addr: *mut sockaddr,
                     addrlen: *mut socklen_t)
                     -> ssize_t {
        syscall!(SOCKETCALL,
                 SYS_ACCEPT,
                 &[fd as c_long,
                   addr as c_long,
                   addrlen as c_long,
                   0,
                   0,
                   0] as *const _) as ssize_t
    }
    accept(fd, addr, addrlen)
}

// net/socket.c
#[inline(always)]
pub unsafe fn accept4(fd: c_int,
                      addr: *mut sockaddr,
                      addrlen: *mut socklen_t,
                      flags: c_int)
                      -> ssize_t {
    #[cfg(not(target_arch = "x86"))]
    #[inline(always)]
    unsafe fn accept4(fd: c_int,
                      addr: *mut sockaddr,
                      addrlen: *mut socklen_t,
                      flags: c_int)
                      -> ssize_t {
        syscall!(ACCEPT4, fd, addr, addrlen, flags) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn accept4(fd: c_int,
                      addr: *mut sockaddr,
                      addrlen: *mut socklen_t,
                      flags: c_int)
                      -> ssize_t {
        syscall!(SOCKETCALL,
                 SYS_ACCEPT4,
                 &[fd as c_long,
                   addr as c_long,
                   addrlen as c_long,
                   flags,
                   0,
                   0] as *const _) as ssize_t
    }
    accept4(fd, addr, addrlen, flags)
}

// net/socket.c
#[inline(always)]
pub unsafe fn setsockopt(fd: c_int,
                         level: c_int,
                         name: c_int,
                         value: *const c_uchar,
                         optlen: socklen_t)
                         -> ssize_t {
    #[cfg(not(target_arch = "x86"))]
    #[inline(always)]
    unsafe fn setsockopt(fd: c_int,
                         level: c_int,
                         name: c_int,
                         value: *const c_uchar,
                         optlen: socklen_t)
                         -> ssize_t {
        syscall!(SETSOCKOPT, fd, level, name, value, optlen) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn setsockopt(fd: c_int,
                         level: c_int,
                         name: c_int,
                         value: *const c_uchar,
                         optlen: socklen_t)
                         -> ssize_t {
        syscall!(SOCKETCALL,
                 SYS_SETSOCKOPT,
                 &[fd as c_long,
                   level,
                   name,
                   value as c_long,
                   optlen,
                   0] as *const _) as ssize_t
    }
    setsockopt(fd, level, name, value, optlen)
}

// net/socket.c
#[inline(always)]
pub unsafe fn getsockopt(fd: c_int,
                         level: c_int,
                         name: c_int,
                         value: *mut c_uchar,
                         optlen: *mut socklen_t)
                         -> ssize_t {
    #[cfg(not(target_arch = "x86"))]
    #[inline(always)]
    unsafe fn getsockopt(fd: c_int,
                         level: c_int,
                         name: c_int,
                         value: *mut c_uchar,
                         optlen: *mut socklen_t)
                         -> ssize_t {
        syscall!(GETSOCKOPT, fd, level, name, value, optlen) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn getsockopt(fd: c_int,
                         level: c_int,
                         name: c_int,
                         value: *mut c_uchar,
                         optlen: *mut socklen_t)
                         -> ssize_t {
        syscall!(SOCKETCALL,
                 SYS_GETSOCKOPT,
                 &[fd as c_long,
                   level,
                   name,
                   value as c_long,
                   optlen as c_long,
                   0] as *const _) as ssize_t
    }
    getsockopt(fd, level, name, value, optlen)
}

// net/socket.c
#[inline(always)]
pub unsafe fn shutdown(fd: c_int, how: c_int) -> ssize_t {
    #[cfg(not(target_arch = "x86"))]
    #[inline(always)]
    unsafe fn shutdown(fd: c_int, how: c_int) -> ssize_t {
        syscall!(SHUTDOWN, fd, how) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn shutdown(fd: c_int, how: c_int) -> ssize_t {
        syscall!(SOCKETCALL,
                 SYS_SHUTDOWN,
                 &[fd as c_long,
                   how,
                   0,
                   0,
                   0,
                   0] as *const _) as ssize_t
    }
    shutdown(fd, how)
}

// net/socket.c
#[inline(always)]
pub unsafe fn connect(fd: c_int,
                      addr: *const sockaddr,
                      addrlen: socklen_t)
                      -> ssize_t {
    #[cfg(not(target_arch = "x86"))]
    #[inline(always)]
    unsafe fn connect(fd: c_int,
                      addr: *const sockaddr,
                      addrlen: socklen_t)
                      -> ssize_t {
        syscall!(CONNECT, fd, addr, addrlen) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn connect(fd: c_int,
                      addr: *const sockaddr,
                      addrlen: socklen_t)
                      -> ssize_t {
        syscall!(SOCKETCALL,
                 SYS_CONNECT,
                 &[fd as c_long,
                   addr as c_long,
                   addrlen,
                   0,
                   0,
                   0] as *const _) as ssize_t
    }
    connect(fd, addr, addrlen)
}

// net/socket.c
#[inline(always)]
pub unsafe fn send(fd: c_int,
                   buf: *const c_uchar,
                   len: size_t,
                   flags: c_int)
                   -> ssize_t {
    sendto(fd, buf, len, flags, 0 as *const _, 0)
}

// net/socket.c
#[inline(always)]
pub unsafe fn sendto(fd: c_int,
                     buf: *const c_uchar,
                     len: size_t,
                     flags: c_int,
                     addr: *const sockaddr,
                     addrlen: socklen_t)
                     -> ssize_t {
    #[cfg(not(target_arch = "x86"))]
    #[inline(always)]
    unsafe fn sendto(fd: c_int,
                     buf: *const c_uchar,
                     len: size_t,
                     flags: c_int,
                     addr: *const sockaddr,
                     addrlen: socklen_t)
                     -> ssize_t {
        syscall!(SENDTO, fd, buf, len, flags, addr, addrlen) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn sendto(fd: c_int,
                     buf: *const c_uchar,
                     len: size_t,
                     flags: c_int,
                     addr: *const sockaddr,
                     addrlen: socklen_t)
                     -> ssize_t {
        syscall!(SOCKETCALL,
                 SYS_SENDTO,
                 &[fd as c_long,
                   buf as c_long,
                   len as c_long,
                   flags,
                   addr as c_long,
                   addrlen] as *const _) as ssize_t
    }
    sendto(fd, buf, len, flags, addr, addrlen)
}

// net/socket.c
#[inline(always)]
pub unsafe fn getpeername(fd: c_int,
                          addr: *mut sockaddr,
                          addrlen: *mut socklen_t)
                          -> ssize_t {
    #[cfg(not(target_arch = "x86"))]
    #[inline(always)]
    unsafe fn getpeername(fd: c_int,
                          addr: *mut sockaddr,
                          addrlen: *mut socklen_t)
                          -> ssize_t {
        syscall!(GETPEERNAME, fd, addr, addrlen) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn getpeername(fd: c_int,
                          addr: *mut sockaddr,
                          addrlen: *mut socklen_t)
                          -> ssize_t {
        syscall!(SOCKETCALL,
                 SYS_GETPEERNAME,
                 &[fd as c_long,
                   addr as c_long,
                   addrlen as c_long,
                   0,
                   0,
                   0] as *const _) as ssize_t

    }
    getpeername(fd, addr, addrlen)
}

// net/socket.c
#[inline(always)]
pub unsafe fn getsockname(fd: c_int,
                          addr: *mut sockaddr,
                          addrlen: *mut socklen_t)
                          -> ssize_t {
    #[cfg(not(target_arch = "x86"))]
    #[inline(always)]
    unsafe fn getsockname(fd: c_int,
                          addr: *mut sockaddr,
                          addrlen: *mut socklen_t)
                          -> ssize_t {
        syscall!(GETSOCKNAME, fd, addr, addrlen) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn getsockname(fd: c_int,
                          addr: *mut sockaddr,
                          addrlen: *mut socklen_t)
                          -> ssize_t {
        syscall!(SOCKETCALL,
                 SYS_GETSOCKNAME,
                 &[fd as c_long,
                   addr as c_long,
                   addrlen as c_long,
                   0,
                   0,
                   0] as *const _) as ssize_t
    }
    getsockname(fd, addr, addrlen)
}

// net/socket.c
#[inline(always)]
pub unsafe fn bind(fd: c_int,
                   addr: *const sockaddr,
                   addrlen: socklen_t)
                   -> ssize_t {
    #[cfg(not(target_arch = "x86"))]
    #[inline(always)]
    unsafe fn bind(fd: c_int,
                   addr: *const sockaddr,
                   addrlen: socklen_t)
                   -> ssize_t {
        syscall!(BIND, fd, addr, addrlen) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn bind(fd: c_int,
                   addr: *const sockaddr,
                   addrlen: socklen_t)
                   -> ssize_t {
        syscall!(SOCKETCALL,
                 SYS_BIND,
                 &[fd as c_long,
                   addr as c_long,
                   addrlen,
                   0,
                   0,
                   0] as *const _) as ssize_t
    }
    bind(fd, addr, addrlen)
}

// net/socket.c
#[inline(always)]
pub unsafe fn listen(fd: c_int, backlog: c_int) -> ssize_t {
    #[cfg(not(target_arch = "x86"))]
    #[inline(always)]
    unsafe fn listen(fd: c_int, backlog: c_int) -> ssize_t {
        syscall!(LISTEN, fd, backlog) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn listen(fd: c_int, backlog: c_int) -> ssize_t {
        syscall!(SOCKETCALL,
                 SYS_LISTEN,
                 &[fd as c_long,
                   backlog,
                   0,
                   0,
                   0,
                   0] as *const _) as ssize_t
    }
    listen(fd, backlog)
}

// net/socket.c
#[inline(always)]
pub unsafe fn recvfrom(fd: c_int,
                       buf: *mut c_uchar,
                       size: size_t,
                       flags: c_int,
                       addr: *mut sockaddr,
                       addrlen: *mut socklen_t)
                       -> ssize_t {
    #[cfg(not(target_arch = "x86"))]
    #[inline(always)]
    unsafe fn recvfrom(fd: c_int,
                       buf: *mut c_uchar,
                       size: size_t,
                       flags: c_int,
                       addr: *mut sockaddr,
                       addrlen: *mut socklen_t)
                       -> ssize_t {
        syscall!(RECVFROM, fd, buf, size, flags, addr, addrlen) as ssize_t
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    unsafe fn recvfrom(fd: c_int,
                       buf: *mut c_uchar,
                       size: size_t,
                       flags: c_int,
                       addr: *mut sockaddr,
                       addrlen: *mut socklen_t)
                       -> ssize_t {
        syscall!(SOCKETCALL,
                 SYS_RECVFROM,
                 &[fd as c_long,
                   buf as c_long,
                   size as c_long,
                   flags,
                   addr as c_long,
                   addrlen as c_long] as *const _) as ssize_t
    }
    recvfrom(fd, buf, size, flags, addr, addrlen)
}
