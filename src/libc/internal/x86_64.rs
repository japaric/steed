use libc::*;
use linux;

#[cfg(not(test))]
mod not_test {
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

    global_asm!("
        .globl __steed_clone
        __steed_clone:

        and $-16,%rsi # Align the child stack to 16 bytes.

        # Push `arg` onto the child stack.
        sub $8,%rsi
        mov %rcx,(%rsi)

        # Temporarily store `fn_`
        mov %rdi,%r11

        mov $56,%rax     # CLONE
        mov %rdx,%rdi    # flags
                         # child_stack
        mov %r8,%rdx     # ptid
        mov 8(%rsp),%r10 # ctid
        mov %r9,%r8      # newtls

        mov %r11,%r9     # fn_ (not for the syscall)

        syscall

        # CLONE returns 0 in the child thread, return if we're the parent.
        test %rax,%rax
        jnz 1f

        # Mark the lowest stack frame
        xor %rbp,%rbp

        pop %rdi # arg
        call *%r9 # fn_

        mov %rax,%rdi # status
        mov $60,%rax  # EXIT
        syscall

        # Unreachable.
        hlt

        1:
        ret
    ");
}

pub unsafe fn set_thread_pointer(thread_data: *mut ()) {
    let result = linux::arch_prctl(linux::ARCH_SET_FS, thread_data as c_ulong);
    if result < 0 {
        panic!("set_thread_pointer: arch_prctl: {}", result);
    }
}

#[inline(always)]
pub unsafe fn thread_self() -> *mut thread {
    let result;
    asm!("mov %fs:0,$0":"=r"(result));
    result
}
