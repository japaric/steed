#[cfg(target_arch = "aarch64")]   #[path = "aarch64.rs"]   mod arch;
#[cfg(target_arch = "arm")]       #[path = "arm.rs"]       mod arch;
#[cfg(target_arch = "mips")]      #[path = "mips.rs"]      mod arch;
#[cfg(target_arch = "mips64")]    #[path = "mips64.rs"]    mod arch;
#[cfg(target_arch = "powerpc")]   #[path = "powerpc.rs"]   mod arch;
#[cfg(target_arch = "powerpc64")] #[path = "powerpc64.rs"] mod arch;
#[cfg(target_arch = "sparc64")]   #[path = "sparc64.rs"]   mod arch;
#[cfg(target_arch = "x86")]       #[path = "x86.rs"]       mod arch;
#[cfg(target_arch = "x86_64")]    #[path = "x86_64.rs"]    mod arch;

use super::*;
use self::arch::*;

pub struct Buffer(thread);

pub unsafe fn init_main_thread(buffer: *mut Buffer) {
    let buffer: *mut thread = &mut (*buffer).0;
    *buffer = thread {
        this: buffer,
        thread_id: -1,
    };
    set_thread_pointer(buffer as *mut _);
}
