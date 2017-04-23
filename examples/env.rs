use std::env;
fn main() {
    println!("{}", env::current_exe().unwrap().display());
    println!("{}", env::temp_dir().display());
    for (k, v) in env::vars_os() {
        println!("{:?}: {:?}", k, v);
    }
}
