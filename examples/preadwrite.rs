#![feature(file_offset)]

use std::fs::OpenOptions;
use std::io::{self, Write};
use std::os::unix::fs::FileExt;

pub fn main() {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("/target/readwrite")
        .unwrap();
    f.write_at(b"Hi?\n", 0).unwrap();
    let mut buffer = [0; 256];
    let n = f.read_at(&mut buffer, 0).unwrap();
    io::stdout().write_all(&buffer[..n]).unwrap();
}
