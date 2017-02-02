use std::process::Command;

fn main() {
    let result = Command::new("/bin/echo")
        .arg("Execution of /bin/echo successful")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    assert!(result.success());
}
