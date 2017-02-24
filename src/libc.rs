#![allow(non_camel_case_types)]

use cmp;
use linux;
use mem;
use ptr;

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

pub use linux::{accept, accept4, bind, close, connect, fdatasync, fstat64};
pub use linux::{fsync, ftruncate64, getpeername, getsockname, getsockopt};
pub use linux::{ioctl, link, listen, lstat64, mmap, nanosleep, prctl, pread64};
pub use linux::{pwrite64, read, recvfrom, rename, rmdir, sched_yield, send};
pub use linux::{sendto, setsockopt, socket, socketpair, shutdown, symlink};
pub use linux::{unlink, write};

pub type mode_t = u32;

// Rust 1.15.0
// src/liblibc/src/unix/notbsd/linux/mod.rs
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

pub mod internal {
    use linux;
    use super::*;

    pub struct Buffer(thread);

    #[cfg(target_arch = "arm")]
    unsafe fn set_thread_pointer(thread_data: *mut thread) {
    }

    #[cfg(target_arch = "x86")]
    unsafe fn set_thread_pointer(thread_data: *mut thread) {
        let mut user_desc = linux::user_desc {
            entry_number: -1i32 as u32,
            base_addr: thread_data as u32,
            limit: 0xfffff,
            flags: 0x51,
        };
        let result = linux::set_thread_area(&mut user_desc);
        if result < 0 {
            panic!("set_thread_pointer: set_thread_area: {}", result);
        }
        asm!("mov $0,%gs"::"r"(((user_desc.entry_number << 3) | 3) as u16));
    }

    #[cfg(target_arch = "x86_64")]
    unsafe fn set_thread_pointer(thread_data: *mut thread) {
        let result = linux::arch_prctl(linux::ARCH_SET_FS, thread_data as c_ulong);
        if result < 0 {
            panic!("set_thread_pointer: arch_prctl: {}", result);
        }
    }

    pub unsafe fn init_main_thread(buffer: *mut Buffer) {
        let buffer: *mut thread = &mut (*buffer).0;
        *buffer = thread {
            this: buffer,
            thread_id: -1,
        };
        set_thread_pointer(buffer);
    }
}

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

extern {
    #[link_name = "__steed_clone"]
    fn syscall_clone(fn_: extern "C" fn(*mut c_void) -> *mut c_void,
                     child_stack: *mut c_void,
                     flags: c_ulong,
                     arg: *mut c_void,
                     ptid: *mut pid_t,
                     newtls: *mut c_void,
                     ctid: *mut pid_t) -> pid_t;
}

#[cfg(target_arch = "arm")]
#[inline(never)]
#[naked]
#[no_mangle]
unsafe extern "C" fn __steed_clone() {
    // Syscall number is passed in r7, syscall arguments in r0, r1, r2, r3, r4.
    // The arguments are
    // (flags: c_ulong,           // r0
    //  child_stack: *mut c_void, // r1
    //  ptid: *mut c_int,         // r2
    //  newtls: c_ulong,          // r3
    //  ctid: *mut c_int)         // r4
    //
    // No registers are clobbered, r0 gets the return value.
    //
    // Only r0, r1, r2, r3 are caller-saved, so we must restore the value of
    // all other registers before returning.
    //
    // The calling convention passes arguments on the stack, right to left.
    // Since we push r4 to r7 onto the stack in the very beginning, all offsets
    // are increased by 16. The arguments are
    // (fn_: extern "C" fn(*mut c_void) -> *mut c_void, // r0
    //  child_stack: *mut c_void,                       // r1
    //  flags: c_ulong,                                 // r2
    //  arg: *mut c_void,                               // r3
    //  ptid: *mut pid_t,                               // [sp,#16]
    //  newtls: *mut c_void,                            // [sp,#20]
    //  ctid: *mut pid_t)                               // [sp,#24]
    //
    // Both ABIs return the function result in r0.
    //
    // This means we need the following moves:
    // r2       -> r0 // flags
    // r1       -> r1 // child_stack
    // [sp,#16] -> r2 // ptid
    // [sp,#20] -> r3 // newtls
    // [sp,#24] -> r4 // ctid
    //
    // We save `fn_` in r5, `arg` in r6.

    asm!("
        @ Save r4 to r7
        stmfd sp!,{r4,r5,r6,r7}

        mov r5,r0 @ fn_
        mov r6,r3 @ arg

        mov r7,#120     @ CLONE
        mov r0,r2       @ flags
                        @ child_stack
        ldr r2,[sp,#16] @ ptid
        ldr r3,[sp,#20] @ newtls
        ldr r4,[sp,#24] @ ctid

        and r1,r1,#-16  @ Align the stack

        @ Do the syscall
        svc 0

        @ CLONE returns 0 in the child thread, return if we're the parent.
        tst r0,r0
        bne __steed_clone_parent

        mov r0,r6 @ arg

        @ Do we need to execute `fn_` in thumb mode?
        tst r5,#1
        bne __steed_clone_thumb

        @ pc (Program Counter) is always 2 instructions ahead.
        mov lr,pc
        mov pc,r5 @ fn_

        @ The function will return here.
        __steed_clone_exit:
        mov r7,#1 @ EXIT
                  @ status
        svc 0

        __steed_clone_thumb:

        @ Again, pc is 2 instructions ahead.
        mov lr,pc
        bx r5 @ Start thumb mode
        b __steed_clone_exit

        __steed_clone_parent:

        @ Restore r4 to r7
        ldmfd sp!,{r4,r5,r6,r7}
    ");
}

#[cfg(target_arch = "x86")]
#[inline(never)]
#[naked]
#[no_mangle]
unsafe extern "C" fn __steed_clone() {
    // Syscall number is passed in %eax, syscall arguments in %ebx, %ecx, %edx,
    // %esi, %edi. The arguments are
    // (flags: c_ulong,           // %ebx
    //  child_stack: *mut c_void, // %ecx
    //  ptid: *mut c_int,         // %edx
    //  newtls: c_ulong,          // %esi
    //  ctid: *mut c_int)         // %edi
    //
    // No registers are clobbered, %eax gets the return value.
    //
    // Only %eax, %ecx and %edx are caller-saved, so we must restore the value
    // of all other registers before returning.
    //
    // The cdecl calling convention passes arguments on the stack, right to
    // left. Since we push %ebp onto the stack in the very beginning, all
    // offsets are increased by 4. The arguments are
    // (fn_: extern "C" fn(*mut c_void) -> *mut c_void, // 8(%ebp)
    //  child_stack: *mut c_void,                       // 12(%ebp)
    //  flags: c_ulong,                                 // 16(%ebp)
    //  arg: *mut c_void,                               // 20(%ebp)
    //  ptid: *mut pid_t,                               // 24(%ebp)
    //  newtls: *mut c_void,                            // 28(%ebp)
    //  ctid: *mut pid_t)                               // 32(%ebp)
    //
    // Both ABIs return the function result in %eax.
    //
    // This means we need the following moves:
    // 16(%ebp) -> %ebx // flags
    // 12(%ebp) -> %ecx // child_stack
    // 24(%ebp) -> %edx // ptid
    // fancy    -> %esi // newtls
    // 32(%ebp) -> %edi // ctid
    //
    // We need to create a struct of type `struct user_desc` (see `clone(2)`
    // and `set_thread_area(2)`) and store it in %esi. We do it by pushing it
    // onto the parent stack.
    //
    // We save `fn_` in %ebp.

    asm!("
        # Stack frame
        push %ebp
        mov %esp,%ebp

        # Save registers
        push %ebx
        push %esi
        push %edi

        mov 12(%ebp),%ecx # child_stack
        and $$-16,%ecx    # Align the stack

        # Push the parameter
        sub $$16,%ecx     # Keep the stack aligned
        mov 20(%ebp),%edi # arg
        mov %edi,(%ecx)

        # Construct the struct
        # I don't know what these parameters do, but glibc and musl agree on
        # these.

        # Bitfield, according to glibc:
        # seg_32bit:1 = 1
        # contents:2 = 0
        # read_exec_only:1 = 0
        # limit_in_pages:1 = 1
        # seg_not_present:1 = 0
        # useable:1 = 1
        push $$0x51
        push $$0xfffff # limit
        push 28(%ebp)  # base_addr
        xor %eax,%eax
        mov %gs,%ax
        shr $$3,%eax
        push %eax      # entry_number

        mov $$120,%eax    # CLONE
        mov 16(%ebp),%ebx # flags
        mov 24(%ebp),%edx # ptid
        mov %esp,%esi     # newtls
        mov 32(%ebp),%edi # ctid

        mov 8(%ebp),%ebp  # fn_

        int $$0x80

        # CLONE returns 0 in the child thread, return if we're the parent.
        test %eax,%eax
        jnz __steed_clone_parent

        mov %ebp,%eax # fn_

        # Mark the lowest stack frame
        xor %ebp,%ebp

        # arg is already on the stack
        call *%eax

        mov %eax,%ebx # status
        mov $$1,%eax  # EXIT
        int $$0x80
        hlt

        __steed_clone_parent:

        # Pop the struct
        add $$16,%esp

        # Restore registers
        pop %edi
        pop %esi
        pop %ebx

        # Stack frame
        pop %ebp
    ");
}

#[cfg(target_arch = "x86_64")]
#[inline(never)]
#[naked]
#[no_mangle]
unsafe extern "C" fn __steed_clone() {
    // Syscall number is passed in %rax, syscall arguments in %rdi, %rsi, %rdx,
    // %r10, %r8. The arguments are
    // (flags: c_ulong,           // %rdi
    //  child_stack: *mut c_void, // %rsi
    //  ptid: *mut c_int,         // %rdx
    //  ctid: *mut c_int          // %r10
    //  newtls: c_ulong)          // %r8
    //
    // The registers %rcx and %r11 are clobbered, %rax gets the return value.
    //
    // The System V AMD64 ABI passes arguments in %rdi, %rsi, %rdx, %rcx, %r8,
    // %r9, 8(%rsp). The arguments are
    // (fn_: extern "C" fn(*mut c_void) -> *mut c_void, // %rdi
    //  child_stack: *mut c_void,                       // %rsi
    //  flags: c_ulong,                                 // %rdx
    //  arg: *mut c_void,                               // %rcx
    //  ptid: *mut pid_t,                               // %r8
    //  newtls: *mut c_void,                            // %r9
    //  ctid: *mut pid_t)                               // 8(%rsp)
    //
    // Both ABIs return the function result in %rax.
    //
    // This means we need the following moves:
    // %rdx    -> %rdi // flags
    // %rsi    -> %rsi // child_stack
    // %r8     -> %rdx // ptid
    // 8(%rsp) -> %r10 // ctid
    // %r9     -> %r8  // newtls
    //
    // And to save `fn_`, we need
    // %rdi    -> %r9  // fn_
    //
    // There's a cycle in there (%rdx -> %rdi -> %r9 -> %r8 -> %rdx), we break
    // it at %rdi -> %r9 and move it to the scratch register %r11 instead.

    asm!("
        and $$-16,%rsi # Align the child stack to 16 bytes.

        # Push `arg` onto the child stack.
        sub $$8,%rsi
        mov %rcx,(%rsi)

        # Temporarily store `fn_`
        mov %rdi,%r11

        mov $$56,%rax    # CLONE
        mov %rdx,%rdi    # flags
                         # child_stack
        mov %r8,%rdx     # ptid
        mov 8(%rsp),%r10 # ctid
        mov %r9,%r8      # newtls

        mov %r11,%r9     # fn_ (not for the syscall)

        syscall

        # CLONE returns 0 in the child thread, return if we're the parent.
        test %rax,%rax
        jnz __steed_clone_parent

        # Mark the lowest stack frame
        xor %rbp,%rbp

        pop %rdi # arg
        call *%r9 # fn_

        mov %rax,%rdi # status
        mov $$60,%rax # EXIT
        syscall

        # Unreachable.
        hlt

        __steed_clone_parent:
    ");
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

pub unsafe fn pthread_join(pthread: pthread_t, retval: *mut *mut c_void)
    -> c_int
{
    assert!(retval.is_null());
    let thread = pthread.thread;

    let tmp = (*thread).thread_id;
    if tmp == 0 {
        return 0;
    }
    // TODO(steed): Why does FUTEX_WAIT_PRIVATE not work?
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

// Rust 1.15.0: src/liblibc/src/unix/notbsd/mod.rs
#[allow(non_snake_case)]
pub fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

// Rust 1.15.0: src/liblibc/src/unix/notbsd/mod.rs
#[allow(non_snake_case)]
pub fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

// Rust 1.15.0: src/liblibc/src/unix/notbsd/mod.rs
#[allow(non_snake_case)]
pub fn WEXITSTATUS(status: c_int) -> c_int {
    (status >> 8) & 0xff
}
