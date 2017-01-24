use std::fs;
use std::path::Path;

pub fn main() {
    for f in fs::read_dir("/").unwrap() {
        let f = f.unwrap();
        println!("{}", Path::new(&f.file_name()).display());
    }
}
