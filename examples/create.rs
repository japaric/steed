use std::fs::File;
use std::io::Write;

fn main() {
    let mut f = File::create("/target/hello").unwrap();
    f.write_all(b"Hello, world!\n").unwrap();
}
