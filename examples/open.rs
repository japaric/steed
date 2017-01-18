use std::fs::File;
use std::io::{Read, Write};
use std::io;

fn main() {
    let mut f = File::open("/target/hello").unwrap();
    let mut buffer = [0; 256];
    let n = f.read(&mut buffer).unwrap();
    io::stdout().write_all(&buffer[..n]).unwrap();
}
