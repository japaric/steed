#[inline(never)]
#[naked]
#[no_mangle]
unsafe extern "C" fn __steed_clone() {
    // Syscall number is passed in r0, syscall arguments in r3, r4, r5, r6, r7.
    // The arguments are
    // (flags: c_ulong,           // r3
    //  child_stack: *mut c_void, // r4
    //  ptid: *mut c_int,         // r5
    //  newtls: c_ulong,          // r6
    //  ctid: *mut c_int)         // r7
    //
    // No registers are clobbered, r3 gets the return value, the error flag
    // goes into r0.
    //
    // We do not clobber any registers, so we don't need to save any.
    //
    // The calling convention passes arguments int registers, from r3 to r9.
    // (fn_: extern "C" fn(*mut c_void) -> *mut c_void, // r3
    //  child_stack: *mut c_void,                       // r4
    //  flags: c_ulong,                                 // r5
    //  arg: *mut c_void,                               // r6
    //  ptid: *mut pid_t,                               // r7
    //  newtls: *mut c_void,                            // r8
    //  ctid: *mut pid_t)                               // r9
    //
    // Both ABIs return the function result in r3.
    //
    // This means we need the following moves:
    // r5 -> r3 // flags
    // r4 -> r4 // child_stack
    // r7 -> r5 // ptid
    // r8 -> r6 // newtls
    // r9 -> r7 // ctid
    //
    // We save `fn_` and `arg` in r30 and r31 which we need to save first.

    asm!("
        # Save `fn_` and `arg` (while keeping the stack aligned).
        stwu 30,-16(1)
        stw 31,4(1)
        mr 30,3 # fn_
        mr 31,6 # arg

        # Align the stack.
        clrrwi 4,4,4  # Store in r4, load from r4, clear last 4 bits
        # Mark the lowest stack frame.
        li 0,0
        stwu 0,-16(4)

        li 0,120 # CLONE
        mr 3,5   # flags
                 # child_stack
        mr 5,7   # ptid
        mr 6,8   # newtls
        mr 7,9   # ctid
        sc

        # PowerPC errno post-processing
        bso- 1f

        # CLONE returns 0 in child, return if we're parent.
        cmpwi cr7,3,0
        bne cr7,2f

        # We're the child.
        mr 3,31  # arg
        mtctr 30 # fn_ into the ctr register
        bctrl    # Call fn_

        li 0,1 # SYS_EXIT
               # status
        sc

        # Parent
        # Negate result if error occured.
        1:
        neg 3,3
        2:

        # Restore saved registers
        lwz 30,0(1)
        lwz 31,4(1)
        addi 1,1,16
    ");
}

pub unsafe fn set_thread_pointer(thread_data: *mut ()) {
    let _ = thread_data; // TODO(steed, #127): Set thread-local pointer.
}
