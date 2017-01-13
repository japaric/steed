use linux;

pub fn exit(code: i32) -> ! {
    unsafe { linux::exit(code) }
}
