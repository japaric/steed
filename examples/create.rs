#![no_std]
#![no_main]

#[macro_use]
extern crate std;

use std::fs::File;
use std::io::Write;

#[no_mangle]
pub fn main() -> i32 {
    let mut f = File::create(b"/target/hello\0").unwrap();
    f.write_all(b"Hello, world!\n").unwrap();
    0
}
