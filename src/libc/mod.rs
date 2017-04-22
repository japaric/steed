#![allow(non_camel_case_types)]

#![cfg_attr(not(any(target_arch = "aarch64",
                    target_arch = "arm",
                    target_arch = "powerpc",
                    target_arch = "x86",
                    target_arch = "x86_64")),
            allow(unused))]

use borrow::Cow;
use cmp;
use collections::HashMap;
use ffi::CStr;
use linux;
use mem;
use memchr::memchr;
use ptr;

pub mod internal;

pub use ctypes::*;
pub use linux::errno::*;

pub use linux::{gid_t, in_addr, in6_addr, ip_mreq, ipv6_mreq, off_t, off64_t};
pub use linux::{pid_t, sa_family_t, sockaddr, sockaddr_in, sockaddr_in6};
pub use linux::{sockaddr_storage, sockaddr_un, socklen_t, stat64, suseconds_t};
pub use linux::{time_t, timespec, timeval, uid_t};

pub use linux::{AF_INET, AF_INET6, AF_UNIX};
pub use linux::{CLONE_CHILD_CLEARTID, CLONE_FILES, CLONE_FS};
pub use linux::{CLONE_PARENT_SETTID, CLONE_SETTLS, CLONE_SIGHAND};
pub use linux::{CLONE_SYSVSEM, CLONE_THREAD, CLONE_VM};
pub use linux::{DT_BLK, DT_CHR, DT_DIR, DT_FIFO, DT_LNK, DT_REG, DT_SOCK};
pub use linux::{F_DUPFD_CLOEXEC, F_DUPFD, F_GETFL, F_SETFL};
pub use linux::{FIOCLEX, FIONBIO};
pub use linux::{IP_ADD_MEMBERSHIP, IP_DROP_MEMBERSHIP, IP_MULTICAST_LOOP};
pub use linux::{IP_MULTICAST_TTL, IP_TTL};
pub use linux::{IPV6_ADD_MEMBERSHIP, IPV6_DROP_MEMBERSHIP};
pub use linux::{IPV6_MULTICAST_LOOP, IPV6_V6ONLY};
pub use linux::{IPPROTO_IP, IPPROTO_IPV6, IPPROTO_TCP};
pub use linux::{MAP_ANONYMOUS, MAP_PRIVATE};
pub use linux::{MSG_NOSIGNAL};
pub use linux::{O_ACCMODE, O_APPEND, O_CLOEXEC, O_CREAT, O_DIRECTORY, O_EXCL};
pub use linux::{O_LARGEFILE, O_NONBLOCK, O_PATH, O_RDONLY, O_RDWR, O_TRUNC};
pub use linux::{O_WRONLY};
pub use linux::{PROT_READ, PROT_WRITE};
pub use linux::{S_IFMT, S_IFSOCK, S_IFLNK, S_IFREG, S_IFBLK, S_IFDIR, S_IFCHR};
pub use linux::{S_IFIFO};
pub use linux::{SHUT_RD, SHUT_RDWR, SHUT_WR};
pub use linux::{SO_BROADCAST, SO_ERROR, SO_RCVTIMEO, SO_REUSEADDR};
pub use linux::{SO_SNDTIMEO};
pub use linux::{SOCK_CLOEXEC, SOCK_DGRAM, SOCK_STREAM};
pub use linux::{SOL_SOCKET};
pub use linux::{SEEK_CUR, SEEK_END, SEEK_SET};
pub use linux::{TCP_NODELAY};

pub use linux::{accept, accept4, bind, chdir, close, connect, exit_group};
pub use linux::{fdatasync, fstat64, fsync, ftruncate64, getcwd, getpeername};
pub use linux::{getsockname, getsockopt, ioctl, link, listen, lstat64, mmap};
pub use linux::{nanosleep, prctl, pread64, pwrite64, read, recvfrom, rename};
pub use linux::{rmdir, sched_yield, send, sendto, setsockopt, socket};
pub use linux::{socketpair, shutdown, symlink, unlink, write};

pub type mode_t = u32;

// Rust 1.16.0: src/liblibc/src/unix/notbsd/linux/mod.rs
#[cfg(issue = "22")]
pub const EAI_SYSTEM: c_int = -11;

// Rust 1.15.0
// src/liblibc/src/unix/notbsd/linux/musl/mod.rs
pub const PTHREAD_STACK_MIN: size_t = 2048;

pub const MAP_ANON: c_int = MAP_ANONYMOUS;

pub unsafe fn strlen(cs: *const c_char) -> size_t {
    let mut cs = cs;
    let mut count = 0;
    while *cs != 0 {
        cs = cs.offset(1);
        count += 1;
    }
    count
}

pub struct EnvVal {
    pub key: Cow<'static, [u8]>,
    pub value: Cow<'static, [u8]>,
}

#[allow(unused)]
static mut ENV_PTR: *const *const u8 = 0 as *const *const u8;
static mut ENV: *mut HashMap<&'static [u8], EnvVal> = 0 as *mut _;
#[allow(unused)]
static mut AUXVAL: *const usize = 0 as *const usize;

fn parse_env_str(input: &'static CStr) -> Option<(&'static [u8], &'static [u8])> {
    let input = input.to_bytes();
    if input.is_empty() {
        return None;
    }
    let pos = memchr(b'=', &input[1..]).map(|p| p + 1);
    pos.map(|p| (&input[..p], &input[p+1..]))
}

fn invalid_env_key(key: &[u8]) -> bool {
    // NUL is checked for at a higher level.
    key.is_empty() || memchr(b'=', &key[1..]).is_some()
}

unsafe fn make_static(s: &[u8]) -> &'static [u8] {
    &*(s as *const [u8])
}

/// One-time global initialization.
pub unsafe fn _init(argc: isize, argv: *const *const u8) {
    ENV_PTR = argv.offset(argc + 1) as *const *const u8;
    let mut ptr = ENV_PTR;
    ENV = Box::into_raw(Box::new(HashMap::new()));
    while !(*ptr).is_null() {
        if let Some((key, value)) = parse_env_str(CStr::from_ptr(*ptr as *const _)) {
            if !(*ENV).contains_key(key) {
                (*ENV).insert(key, EnvVal {
                    key: Cow::from(key),
                    value: Cow::from(value),
                });
            }
        }
        ptr = ptr.offset(1);
    }
    AUXVAL = ptr.offset(1) as *const usize;
}

pub unsafe fn getenv(key: &[u8]) -> Option<&'static [u8]> {
    (*ENV).get(key).map(|v| make_static(&v.value))
}

pub unsafe fn setenv(key: &[u8], value: &[u8]) -> c_int {
    if invalid_env_key(key) {
        return -linux::errno::EINVAL;
    }
    let key = Cow::from(key.to_owned());
    let value = Cow::from(value.to_owned());
    let key_static = make_static(&key);
    (*ENV).insert(key_static, EnvVal { key: key, value: value });
    0
}

pub unsafe fn unsetenv(key: &[u8]) -> c_int {
    if invalid_env_key(key) {
        return -linux::errno::EINVAL;
    }
    (*ENV).remove(key);
    0
}

pub unsafe fn env() -> &'static HashMap<&'static [u8], EnvVal> {
    &*ENV
}

pub unsafe fn environ() -> *const *const c_char {
    static ENVIRON: [usize; 1] = [0];
    ENVIRON.as_ptr() as *const _
}

#[inline(always)]
pub unsafe fn fcntl(fd: c_int, cmd: c_uint, arg: c_int) -> c_int {
    linux::fcntl(fd, cmd, arg as c_ulong) as c_int
}

#[inline(always)]
pub unsafe fn open64(pathname: *const c_char, flags: c_int, mode: c_int) -> c_int {
    linux::open(pathname, flags | O_LARGEFILE, mode as linux::umode_t)
}

#[inline(always)]
pub unsafe fn readlink(pathname: *const c_char,
                       buf: *mut c_char,
                       bufsiz: size_t)
    -> ssize_t
{
    linux::readlink(pathname,
                    buf,
                    cmp::min(bufsiz, <c_int>::max_value() as size_t) as c_int)
}

#[inline(always)]
pub unsafe fn fchmod(fd: c_int, mode: mode_t) -> c_int {
    linux::fchmod(fd, mode as linux::umode_t)
}

#[inline(always)]
pub unsafe fn chmod(filename: *const c_char, mode: mode_t) -> c_int {
    linux::chmod(filename, mode as linux::umode_t)
}

#[inline(always)]
pub unsafe fn mkdir(pathname: *const c_char, mode: mode_t) -> c_int {
    linux::mkdir(pathname, mode as linux::umode_t)
}

#[inline(always)]
pub unsafe fn lseek64(fd: c_int, offset: off64_t, whence: c_uint) -> off64_t {
    let mut result_offset: off64_t = 0;
    let result = linux::_llseek(fd, offset, &mut result_offset, whence);
    if result >= 0 {
        result_offset
    } else {
        result as off64_t
    }
}

#[derive(Clone, Copy)]
pub struct pthread_attr_t {
    stack_size: usize,
}

struct thread {
    // Required, because one cannot easily read out register containing the
    // pointer to this structure on some platforms.
    this: *mut thread,
    thread_id: pid_t,
}

#[derive(Clone, Copy)]
pub struct pthread_t {
    thread: *mut thread,
}

pub unsafe fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int {
    *attr = pthread_attr_t {
        stack_size: 0,
    };
    0
}

pub unsafe fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int {
    pthread_attr_init(attr)
}

pub unsafe fn pthread_attr_setstacksize(attr: *mut pthread_attr_t,
                                        stacksize: size_t)
    -> c_int
{
    (*attr).stack_size = stacksize;
    0
}

/*
pub unsafe fn pthread_attr_getstack(attr: *const pthread_attr_t,
                                    stackaddr: *mut *mut c_void,
                                    stacksize: *mut size_t)
    -> c_int
{
    *stackaddr = ptr::null_mut();
    *stacksize = (*attr).stack_size;
    0
}

pub unsafe fn pthread_attr_getguardsize(attr: *const pthread_attr_t,
                                        guardsize: *mut size_t)
    -> c_int
{
    *guardsize = 0;
    0
}

pub unsafe fn pthread_getattr_np(pthread: pthread_t,
                                 attr: *mut pthread_attr_t)
    -> c_int
{
    pthread_attr_init(attr)
}
*/

#[cfg(not(any(target_arch = "aarch64",
              target_arch = "arm",
              target_arch = "powerpc",
              target_arch = "x86",
              target_arch = "x86_64")))]
pub unsafe fn pthread_create(pthread: *mut pthread_t,
                             attr: *const pthread_attr_t,
                             start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
                             arg: *mut c_void)
    -> c_int
{
    if false {
        let flags = CLONE_VM | CLONE_FS | CLONE_FILES | CLONE_SIGHAND
            | CLONE_THREAD | CLONE_SYSVSEM | CLONE_SETTLS
            | CLONE_CHILD_CLEARTID | CLONE_PARENT_SETTID;
        mmap(ptr::null_mut(), 0, PROT_READ | PROT_WRITE, MAP_PRIVATE |
             MAP_ANON, -1, 0);
    }
    unimplemented!();
}
// Heavily simplified (i.e. less features) version of pthread_create.
// musl: src/thread/pthread_create.c
//
// Doesn't care about signals, thread-local storage and perhaps other things
// yet.
#[cfg(any(target_arch = "aarch64",
          target_arch = "arm",
          target_arch = "powerpc",
          target_arch = "x86",
          target_arch = "x86_64"))]
pub unsafe fn pthread_create(pthread: *mut pthread_t,
                             attr: *const pthread_attr_t,
                             start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
                             arg: *mut c_void)
    -> c_int
{
    let _ = attr;
    let flags = CLONE_VM | CLONE_FS | CLONE_FILES | CLONE_SIGHAND
        | CLONE_THREAD | CLONE_SYSVSEM | CLONE_SETTLS
        | CLONE_CHILD_CLEARTID | CLONE_PARENT_SETTID;

    let align = 16;
    let mask = align - 1;
    let stack_size = ((*attr).stack_size + mask) & !mask;

    // TODO(steed, #131): Make sure this is freed on normal and abnormal thread
    // exit.
    let map = mmap(ptr::null_mut(),
                   stack_size + mem::size_of::<thread>(),
                   PROT_READ | PROT_WRITE,
                   MAP_PRIVATE | MAP_ANON,
                   -1,
                   0);

    // musl: src/internal/__syscall_ret.c
    if map as usize > -4096isize as usize {
        return -(map as c_int);
    }

    let stack = map.offset(stack_size as isize);
    let thread = stack as *mut thread;
    (*thread).this = thread;

    let child_tid = syscall_clone(start_routine,
                                  stack,
                                  flags,
                                  arg,
                                  &mut (*thread).thread_id,
                                  thread as *mut c_void,
                                  &mut (*thread).thread_id);
    if child_tid < 0 {
        return -child_tid;
    }
    *pthread = pthread_t {
        thread: thread,
    };
    0
}

#[cfg(any(target_arch = "aarch64",
          target_arch = "arm",
          target_arch = "powerpc",
          target_arch = "x86",
          target_arch = "x86_64"))]
extern {
    // Defined in internal/<arch>.rs.
    //
    // Does the equivalent of
    //
    // ```
    // let result = CLONE(flags, child_stack, ptid, ctid, newtls);
    // if result == 0 {
    //     EXIT(fn_(arg));
    // } else {
    //     result
    // }
    // ```
    //
    // where CLONE and EXIT are the respective syscalls. Because it mangles
    // with the stack (in two processes at once), it needs to be written in the
    // target's assembly.
    #[link_name = "__steed_clone"]
    fn syscall_clone(fn_: extern "C" fn(*mut c_void) -> *mut c_void,
                     child_stack: *mut c_void,
                     flags: c_ulong,
                     arg: *mut c_void,
                     ptid: *mut pid_t,
                     newtls: *mut c_void,
                     ctid: *mut pid_t) -> pid_t;
}

/*
#[inline(always)]
#[cfg(target_arch = "x86_64")]
unsafe fn thread_self() -> *mut thread {
    let result;
    asm!("mov %fs:0,$0":"=r"(result));
    result
}

#[inline(always)]
pub unsafe fn pthread_self() -> pthread_t {
    pthread_t {
        thread: thread_self(),
    }
}

pub unsafe fn pthread_detach(thread: pthread_t) -> c_int {
    unimplemented!();
}
*/

#[cfg(not(any(target_arch = "aarch64",
              target_arch = "arm",
              target_arch = "powerpc",
              target_arch = "x86",
              target_arch = "x86_64")))]
pub unsafe fn pthread_join(pthread: pthread_t, retval: *mut *mut c_void)
    -> c_int
{
    if false {
        let thread = pthread.thread;
        linux::futex(&mut (*thread).thread_id as *mut _ as *mut u32,
                     linux::FUTEX_WAIT,
                     (*thread).thread_id as u32,
                     ptr::null(),
                     ptr::null_mut(),
                     0);
    }
    unimplemented!();
}

#[cfg(any(target_arch = "aarch64",
          target_arch = "arm",
          target_arch = "powerpc",
          target_arch = "x86",
          target_arch = "x86_64"))]
pub unsafe fn pthread_join(pthread: pthread_t, retval: *mut *mut c_void)
    -> c_int
{
    assert!(retval.is_null());
    let thread = pthread.thread;

    let tmp = (*thread).thread_id;
    // 0 would mean that the thread has exited already (CLONE_CHILD_CLEARTID
    // flag on the clone syscall).
    if tmp == 0 {
        return 0;
    }
    // TODO(steed, #130): Why does FUTEX_WAIT_PRIVATE not work?
    let res = linux::futex(&mut (*thread).thread_id as *mut _ as *mut u32,
                           linux::FUTEX_WAIT,
                           (*thread).thread_id as u32,
                           ptr::null(),
                           ptr::null_mut(),
                           0);
    if res == -EAGAIN {
        return 0;
    }
    if res < 0 {
        return -res;
    }
    0
}

// Rust 1.16.0: src/liblibc/src/unix/notbsd/mod.rs
#[allow(non_snake_case)]
pub fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

// Rust 1.16.0: src/liblibc/src/unix/notbsd/mod.rs
#[allow(non_snake_case)]
pub fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

// Rust 1.16.0: src/liblibc/src/unix/notbsd/mod.rs
#[allow(non_snake_case)]
pub fn WEXITSTATUS(status: c_int) -> c_int {
    (status >> 8) & 0xff
}
