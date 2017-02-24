use std::io::{self, Write};
use std::thread;

fn main() {
    thread::spawn(|| {
        io::stdout().write_all(b"Hello, world!\n").unwrap();
    }).join().unwrap();
}
