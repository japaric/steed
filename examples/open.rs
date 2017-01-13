#![no_std]
#![no_main]

#[macro_use]
extern crate steed;

use steed::fs::File;
use steed::io::{Read, Write};
use steed::io;

#[no_mangle]
pub fn main() -> i32 {
    let mut f = File::open(b"/target/hello\0").unwrap();
    let mut buffer = [0; 256];
    let n = f.read(&mut buffer).unwrap();
    io::stdout().write_all(&buffer[..n]).unwrap();
    0
}
