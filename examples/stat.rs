use std::fs;
use std::fs::File;
use std::io::Write;
use std::process;

pub fn main() {
    let path = "/target/stat";
    let content = b"Test stat\n";
    let size = content.len() as u64;
    let file = File::create(path).unwrap();
    (&file).write_all(b"Test stat\n").unwrap();
    let f = file.metadata().unwrap();
    let s = fs::metadata(path).unwrap();
    let l = fs::symlink_metadata(path).unwrap();
    println!("{} {} {} {}", size, f.len(), s.len(), l.len());
    if f.len() != size || s.len() != size || l.len() != size {
        process::exit(1);
    }
}
