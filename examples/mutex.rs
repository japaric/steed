use std::sync::Mutex;

fn common() -> Mutex<i32> {
    let m = Mutex::new(0);
    {
        *m.lock().unwrap() = 1;
        println!("{}", *m.lock().unwrap());
    }
    m
}

#[cfg(not(any(target_arch = "aarch64",
              target_arch = "arm",
              target_arch = "powerpc",
              target_arch = "x86",
              target_arch = "x86_64")))]
fn main() {
    common();
    println!("success");
}

#[cfg(any(target_arch = "aarch64",
          target_arch = "arm",
          target_arch = "powerpc",
          target_arch = "x86",
          target_arch = "x86_64"))]
fn main() {
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;
    let m = Arc::new(common());
    let m2 = m.clone();

    let guard = m.lock().unwrap();

    let t = thread::spawn(move || {
        let _ = m2.lock().unwrap();
    });

    thread::sleep(Duration::new(1, 0));
    drop(guard);

    t.join().unwrap();
    println!("success");
}
