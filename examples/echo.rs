#![no_std]
#![no_main]

#[macro_use]
extern crate steed;

use steed::io;
use steed::io::{Read, Write};

#[no_mangle]
pub fn main() -> i32 {
    let mut buffer = [0; 256];
    let mut stdin = io::stdin();
    let n = stdin.read(&mut buffer).unwrap();
    io::stdout().write_all(&buffer[..n]).unwrap();
    0
}
