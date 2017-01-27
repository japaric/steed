use super::c_int;

// Rust 1.14.0: src/liblibc/src/unix/notbsd/mod.rs
#[allow(non_snake_case)]
pub fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

// Rust 1.14.0: src/liblibc/src/unix/notbsd/mod.rs
#[allow(non_snake_case)]
pub fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

// Rust 1.14.0: src/liblibc/src/unix/notbsd/mod.rs
#[allow(non_snake_case)]
pub fn WEXITSTATUS(status: c_int) -> c_int {
    (status >> 8) & 0xff
}
