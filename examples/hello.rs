use std::{io, process};
use std::io::Write;

fn main() {
    if io::stdout().write_all(b"Hello, world!\n").is_err() {
        process::exit(1)
    }
}
