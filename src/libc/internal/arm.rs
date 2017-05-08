use libc::*;
use linux;

#[cfg(not(test))]
mod not_test {
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
    // The calling convention passes the first four arguments in r0 to r3 and
    // all further arguments on the stack, right to left. Since we push r4 to
    // r7 onto the stack in the very beginning, all stack offsets are increased
    // by 16. The arguments are
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

    global_asm!("
        .globl __steed_clone
        __steed_clone:

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
        bne 3f

        mov r0,r6 @ arg

        @ Do we need to execute `fn_` in thumb mode?
        tst r5,#1
        bne 2f

        @ pc (Program Counter) is always 2 instructions ahead.
        mov lr,pc
        mov pc,r5 @ fn_

        @ The function will return here.
        __steed_clone_exit:
        mov r7,#1 @ EXIT
                  @ status
        svc 0

        2:

        @ Again, pc is 2 instructions ahead.
        mov lr,pc
        bx r5 @ Start thumb mode
        b 2b

        3:

        @ Restore r4 to r7
        ldmfd sp!,{r4,r5,r6,r7}
        @ Return from the function.
        mov pc,lr
    ");
}

#[inline(always)]
pub unsafe fn set_thread_pointer(thread_data: *mut ()) {
    linux::arm_set_tls(thread_data);
}

#[inline(always)]
pub unsafe fn thread_self() -> *mut thread {
    let result;
    asm!("mrc p15,0,$0,c13,c0,3":"=r"(result));
    result
}
