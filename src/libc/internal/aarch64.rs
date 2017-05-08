use libc::thread;

#[cfg(not(test))]
mod not_test {
    // Syscall number is passed in x8, syscall arguments in x0, x1, x2, x3, x4.
    // The arguments are
    // (flags: c_ulong,           // x0
    //  child_stack: *mut c_void, // x1
    //  ptid: *mut c_int,         // x2
    //  newtls: c_ulong,          // x3
    //  ctid: *mut c_int)         // x4
    //
    // No registers are clobbered, x0 gets the return value.
    //
    // We do not clobber any registers, so we don't need to save any.
    //
    // The calling convention passes arguments in registers, from x0 to x6.
    // (fn_: extern "C" fn(*mut c_void) -> *mut c_void, // x0
    //  child_stack: *mut c_void,                       // x1
    //  flags: c_ulong,                                 // x2
    //  arg: *mut c_void,                               // x3
    //  ptid: *mut pid_t,                               // x4
    //  newtls: *mut c_void,                            // x5
    //  ctid: *mut pid_t)                               // x6
    //
    // Both ABIs return the function result in x0.
    //
    // This means we need the following moves:
    // x2 -> x0 // flags
    // x1 -> x1 // child_stack
    // x4 -> x2 // ptid
    // x5 -> x3 // newtls
    // x6 -> x4 // ctid
    //
    // We save `fn_` and `arg` on the child stack.

    global_asm!("
        .globl __steed_clone
        __steed_clone:

        // Align the child stack.
        and x1,x1,#-16

        // Save `fn_` and `arg` on the child stack.
        stp x0,x3,[x1,#-16]!

        mov x8,#220 // CLONE
        mov x0,x2   // flags
                    // child_stack
        mov x2,x4   // ptid
        mov x3,x5   // newtls
        mov x4,x6   // ctid
        svc #0

        cbnz x0,1f

        // Restore `fn_` and `arg`.
        ldp x1,x0,[sp],#16
        blr x1

        mov x8,#93 // EXIT
                   // status
        svc #0

        1:
        ret
    ");
}

#[inline(always)]
pub unsafe fn set_thread_pointer(thread_data: *mut ()) {
    asm!("msr tpidr_el0,$0"::"r"(thread_data)::"volatile");
}

#[inline(always)]
pub unsafe fn thread_self() -> *mut thread {
    let result;
    asm!("mrs $0,tpidr_el0":"=r"(result));
    result
}
