use linux;

pub fn errno() -> i32 {
    panic!("no C-compatible errno variable");
}

pub fn error_string(errno: i32) -> String {
    super::errno::error_string(errno).map(|s| s.into()).unwrap_or_else(|| {
        format!("Unknown OS error ({})", errno)
    })
}

pub fn exit(code: i32) -> ! {
    unsafe { linux::exit_group(code) }
}
