use std::fmt::format;

use anyhow::{anyhow, Result};

struct TestResult {
    run_count: i32,
    err_count: i32,
}

impl TestResult {
    fn new() -> Self {
        Self {
            run_count: 0,
            err_count: 0,
        }
    }

    fn test_started(&mut self) {
        self.run_count += 1;
    }

    fn test_failed(&mut self) {
        self.err_count += 1;
    }

    fn summary(&self) -> String {
        format!("{} run, {} failed", self.run_count, self.err_count)
    }
}

trait Testable {
    fn run(&self) -> Result<()>;
    fn name(&self) -> String;
}

struct TestMethod;
impl Testable for TestMethod {
    fn run(&self) -> Result<()> {
        Ok(())
    }
    fn name(&self) -> String {
        "test_method".to_owned()
    }
}

struct TestBrokenMethod;
impl Testable for TestBrokenMethod {
    fn run(&self) -> Result<()> {
        Err(anyhow!(""))
    }
    fn name(&self) -> String {
        "test_broken_method".to_owned()
    }
}

struct TestCase {
    method: Box<dyn Testable>,
    log: String,
}

impl TestCase {
    fn new<T>(t: T) -> Self
    where
        T: Testable + 'static,
    {
        Self {
            method: Box::new(t),
            log: "".to_owned(),
        }
    }

    fn setup(&mut self) {
        self.log = "setup ".to_owned();
    }

    fn run(&mut self) -> TestResult {
        let mut result = TestResult::new();
        result.test_started();
        self.setup();
        match self.method.run() {
            Ok(_) => self.log = format!("{}{} ", self.log, self.method.name()),
            Err(_) => result.test_failed(),
        };
        self.teardown();
        result
    }

    fn teardown(&mut self) {
        self.log = format!("{}teardown ", self.log);
    }

    fn test_method(&mut self) -> Result<()> {
        self.log = format!("{}test_method ", self.log);
        Ok(())
    }

    fn test_broken_method(&mut self) -> Result<()> {
        Err(anyhow!(""))
    }
}

struct TestSuite {
    tests: Vec<TestCase>,
}

impl TestSuite {
    fn new() -> Self {
        Self { tests: Vec::new() }
    }

    fn add(&mut self, test: TestCase) {
        self.tests.push(test);
    }

    fn run(&self) -> TestResult {
        let mut result = TestResult::new();
        for test in self.tests.iter() {}
        todo!()
    }
}

struct TestCaseTest {}

impl TestCaseTest {
    fn new() -> Self {
        Self {}
    }

    fn test_template_method(&self) {
        let mut test = TestCase::new(TestMethod);
        test.run();
        assert_eq!("setup test_method teardown ", test.log);
    }

    fn test_result(&self) {
        let mut test = TestCase::new(TestMethod);
        let result = test.run();
        assert_eq!("1 run, 0 failed", result.summary());
    }

    fn test_failed_result(&self) {
        let mut test = TestCase::new(TestBrokenMethod);
        let result = test.run();
        assert_eq!("1 run, 1 failed", result.summary());
    }

    fn test_failed_result_formatting(&self) {
        let mut result = TestResult::new();
        result.test_started();
        result.test_failed();
        assert_eq!("1 run, 1 failed", result.summary());
    }

    fn test_suite(&self) {
        let mut suite = TestSuite::new();
        suite.add(TestCase::new(TestMethod));
        suite.add(TestCase::new(TestBrokenMethod));
        let result = suite.run();
        assert_eq!("2 run, 1 failed", result.summary());
    }
}

#[cfg(test)]
mod tests {
    use crate::xunit::*;

    #[test]
    fn method() {
        let test = TestCaseTest::new();
        test.test_template_method();
        test.test_result();
        test.test_failed_result();
        test.test_failed_result_formatting();
        // test.test_suite();
    }
}
