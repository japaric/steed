pub mod test {
    pub use ShouldPanic;
}

pub struct TestDesc {
    pub ignore: bool,
    pub name: StaticTestName,
    pub should_panic: ShouldPanic,
}

pub struct TestDescAndFn {
    pub desc: TestDesc,
    pub testfn: StaticTestFn,
}

pub struct StaticTestFn(pub fn());
pub struct StaticTestName(pub &'static str);

pub enum ShouldPanic {
    No,
    Yes,
}

pub fn test_main_static(tests: &'static [TestDescAndFn]) {
    let n = tests.len();

    println!("Running {} test{}", n, if n == 1 { "" } else { "s" });

    let mut passed = 0;
    let mut ignored = 0;
    for test in tests {
        match test.desc.should_panic {
            ShouldPanic::No => {
                print!("test {} ... ", test.desc.name.0);
                test.testfn.0();
                println!("ok");
                passed += 1;
            }
            ShouldPanic::Yes => {
                println!("test {} ... ignored", test.desc.name.0);
                ignored += 1;
            }
        }
    }

    println!("\ntest result: ok. {} passed; 0 failed; {} ignored; 0 measured",
             passed,
             ignored);
}
