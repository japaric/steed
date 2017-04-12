use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::io;
use std::os::unix::fs::FileExt;
use std::process;

fn main() {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("/target/dup")
        .unwrap();
    let g = f.try_clone().unwrap();
    (&f).write_all(b"Do you copy?\n").unwrap();
    let mut buffer = [0; 256];
    let n = (&g).read(&mut buffer).unwrap();
    {
        let read = &buffer[..n];
        if read != b"" {
            process::exit(1);
        }
    }
    let n = g.read_at(&mut buffer, 0).unwrap();
    let read = &buffer[..n];
    io::stdout().write_all(read).unwrap();
    if read != b"Do you copy?\n" {
        process::exit(1);
    }
}
