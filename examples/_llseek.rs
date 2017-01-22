use std::fs::OpenOptions;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::process;

pub fn main() {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("/target/_llseek")
        .unwrap();
    (&f).write(b"Can you find me?\n").unwrap();
    let mut buffer = [0; 2];
    (&f).seek(SeekFrom::Current(-4)).unwrap();
    let n = (&f).read(&mut buffer).unwrap();
    let read = &buffer[..n];
    io::stdout().write_all(read).unwrap();
    io::stdout().write_all(b"\n").unwrap();
    if read != b"me" {
        process::exit(1);
    }
}
