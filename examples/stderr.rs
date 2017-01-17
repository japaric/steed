#![no_std]
#![no_main]

#[macro_use]
extern crate std;

use std::io;
use std::io::Write;

#[no_mangle]
pub fn main() -> i32 {
    io::stderr().write_all(b"Hello, world!\n").ok();
    0
}
