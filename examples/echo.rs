use std::io;
use std::io::{Read, Write};

fn main() {
    let mut buffer = [0; 256];
    let mut stdin = io::stdin();
    let n = stdin.read(&mut buffer).unwrap();
    io::stdout().write_all(&buffer[..n]).unwrap();
}
