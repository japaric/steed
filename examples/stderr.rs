use std::io;
use std::io::Write;

fn main() {
    io::stderr().write_all(b"Hello, world!\n").ok();
}
