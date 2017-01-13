#![no_std]
#![no_main]

#[macro_use]
extern crate steed;

#[no_mangle]
pub fn main() -> i32 {
    println!("{:?}", vec![0, 1, 2, 3]);
    0
}
