#![no_std]
#![no_main]

#[macro_use]
extern crate steed;

use steed::fs::File;
use steed::io::Write;

#[no_mangle]
pub fn main() -> i32 {
    let mut f = File::create(b"/target/hello\0").unwrap();
    f.write_all(b"Hello, world!\n").unwrap();
    0
}
