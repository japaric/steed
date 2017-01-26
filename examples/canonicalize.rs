use std::fs;

fn main() {
    println!("{}", fs::canonicalize(".").unwrap().display());
}
