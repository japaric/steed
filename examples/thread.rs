use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        io::stdout().write_all(b"Hello, world!\n").unwrap();
    }).join();
}
