#[cfg(not(any(target_arch = "aarch64",
              target_arch = "arm",
              target_arch = "powerpc",
              target_arch = "x86",
              target_arch = "x86_64")))]
fn main() {
}

#[cfg(any(target_arch = "aarch64",
          target_arch = "arm",
          target_arch = "powerpc",
          target_arch = "x86",
          target_arch = "x86_64"))]
fn main() {
    use std::io::{self, Write};
    use std::thread;

    thread::spawn(|| {
        io::stdout().write_all(b"Hello, world!\n").unwrap();
    }).join().unwrap();
}
