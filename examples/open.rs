#![no_std]
#![no_main]

#[macro_use]
extern crate std;

use std::fs::File;
use std::io::{Read, Write};
use std::io;

#[no_mangle]
pub fn main() -> i32 {
    let mut f = File::open("target/hello").unwrap();
    let mut buffer = [0; 256];
    let n = f.read(&mut buffer).unwrap();
    io::stdout().write_all(&buffer[..n]).unwrap();
    0
}
