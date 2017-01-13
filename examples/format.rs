#![no_std]
#![no_main]

#[macro_use]
extern crate steed;

#[no_mangle]
pub fn main() -> i32 {
    println!("{} + {} = {}", 1, 1, 1 + 1);
    0
}
