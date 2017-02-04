use std::collections::HashMap;

fn main() {
    let mut h = HashMap::new();
    h.insert("foo", "bar");
    h.insert("keks", "baz");
    println!("{:?}", h);
}
