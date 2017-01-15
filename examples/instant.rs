#![no_std]
#![no_main]

#[macro_use]
extern crate steed;

use steed::time::Instant;

#[no_mangle]
pub fn main() -> i32 {
    let now = Instant::now();
    let elapsed = now.elapsed();

    println!("{:?}", elapsed);
    0
}
