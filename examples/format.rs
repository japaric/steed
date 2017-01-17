#![no_std]
#![no_main]

#[macro_use]
extern crate std;

#[no_mangle]
pub fn main() -> i32 {
    println!("{} + {} = {}", 1, 1, 1 + 1);
    0
}
