#![no_std]
#![no_main]

#[macro_use]
extern crate steed;

use steed::io;
use steed::io::Write;

#[no_mangle]
pub fn main() -> i32 {
    io::stderr().write_all(b"Hello, world!\n").ok();
    0
}
