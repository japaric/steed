use std::time::Instant;

fn main() {
    let now = Instant::now();
    let elapsed = now.elapsed();

    println!("{:?}", elapsed);
}
