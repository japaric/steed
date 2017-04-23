use std::fs::OpenOptions;
use std::io::{self, Write};
use std::os::unix::fs::FileExt;
use std::process;

pub fn main() {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("/target/readwrite")
        .unwrap();
    f.write_at(b"She says: Hi?\nWhat do you say?\n", 0).unwrap();
    f.set_len(14).unwrap();
    let mut buffer = [0; 256];
    let n = f.read_at(&mut buffer, 10).unwrap();
    let read = &buffer[..n];
    io::stdout().write_all(read).unwrap();
    if read != b"Hi?\n" {
        process::exit(1);
    }
}
