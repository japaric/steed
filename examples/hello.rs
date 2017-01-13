#![no_std]
#![no_main]

#[macro_use]
extern crate steed;

use steed::io;
use steed::io::Write;

#[no_mangle]
pub fn main() -> i32 {
    io::stdout().write_all(b"Hello, world!\n").map(|_| 0).unwrap_or(1)
}
