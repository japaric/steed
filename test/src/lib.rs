pub use self::TestFn::*;

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
    pub testfn: TestFn,
}

pub enum TestFn {
    StaticBenchFn(fn(&mut Bencher)),
    StaticTestFn(fn()),
}

pub struct StaticTestName(pub &'static str);

pub struct Bencher {}

#[derive(Clone, Copy)]
pub enum ShouldPanic {
    No,
    Yes,
}

pub fn black_box<T>(_: T) {}

pub fn test_main_static(tests: &'static [TestDescAndFn]) {
    let n = tests.len();

    println!("Running {} test{}", n, if n == 1 { "" } else { "s" });

    let mut passed = 0;
    let mut ignored = 0;
    for test in tests {
        match (test.desc.should_panic, &test.testfn) {
            (ShouldPanic::No, &TestFn::StaticTestFn(ref f)) => {
                print!("test {} ... ", test.desc.name.0);
                f();
                println!("ok");
                passed += 1;
            }
            _ => {
                println!("test {} ... ignored", test.desc.name.0);
                ignored += 1;
            }
        }
    }

    println!("\ntest result: ok. {} passed; 0 failed; {} ignored; 0 measured",
             passed,
             ignored);
}
