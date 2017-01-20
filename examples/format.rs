use std::io;
use std::io::Write;

fn main() {
    io::stdout()
        .write_all(format!("{} + {} = {}\n", 1, 1, 1 + 1).as_bytes())
        .unwrap();

}
