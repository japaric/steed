use libc::*;
use linux;

#[cfg(not(test))]
mod not_test {
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

    global_asm!("
        .globl __steed_clone
        __steed_clone:

        # Stack frame
        push %ebp
        mov %esp,%ebp

        # Save registers
        push %ebx
        push %esi
        push %edi

        mov 12(%ebp),%ecx # child_stack
        and $-16,%ecx     # Align the stack

        # Push the parameter
        sub $16,%ecx      # Keep the stack aligned
        mov 20(%ebp),%edi # arg
        mov %edi,(%ecx)

        # Construct a struct of type `user_desc`

        # Bitfield, according to glibc:
        # seg_32bit:1 = 1
        # contents:2 = 0
        # read_exec_only:1 = 0
        # limit_in_pages:1 = 1
        # seg_not_present:1 = 0
        # useable:1 = 1
        push $0x51
        push $0xfffff  # limit
        push 28(%ebp)  # base_addr
        xor %eax,%eax
        mov %gs,%ax
        shr $3,%eax
        push %eax      # entry_number

        mov $120,%eax     # CLONE
        mov 16(%ebp),%ebx # flags
        mov 24(%ebp),%edx # ptid
        mov %esp,%esi     # newtls
        mov 32(%ebp),%edi # ctid

        mov 8(%ebp),%ebp  # fn_

        int $0x80

        # CLONE returns 0 in the child thread, return if we're the parent.
        test %eax,%eax
        jnz 1f

        mov %ebp,%eax # fn_

        # Mark the lowest stack frame
        xor %ebp,%ebp

        # arg is already on the stack
        call *%eax

        mov %eax,%ebx # status
        mov $1,%eax   # EXIT
        int $0x80

        # Unreachable
        hlt

        1:

        # Pop the struct
        add $16,%esp

        # Restore registers
        pop %edi
        pop %esi
        pop %ebx

        # Stack frame
        pop %ebp

        ret
    ");
}

pub unsafe fn set_thread_pointer(thread_data: *mut ()) {
    let mut user_desc = linux::user_desc {
        entry_number: -1i32 as u32,
        base_addr: thread_data as u32,
        limit: 0xfffff,
        // This `flags` value is explained in the `asm!` block of
        // `__steed_clone` above.
        flags: 0x51,
    };
    let result = linux::set_thread_area(&mut user_desc);
    if result < 0 {
        panic!("set_thread_pointer: set_thread_area: {}", result);
    }
    asm!("mov $0,%gs"::"r"(((user_desc.entry_number << 3) | 3) as u16)::"volatile");
}

#[inline(always)]
pub unsafe fn thread_self() -> *mut thread {
    let result;
    asm!("mov %gs:0,$0":"=r"(result));
    result
}
