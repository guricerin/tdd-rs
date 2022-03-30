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

    fn run(&mut self, result: &mut TestResult) {
        result.test_started();
        self.setup();
        match self.method.run() {
            Ok(_) => self.log = format!("{}{} ", self.log, self.method.name()),
            Err(_) => result.test_failed(),
        };
        self.teardown();
    }

    fn teardown(&mut self) {
        self.log = format!("{}teardown ", self.log);
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

    fn run(&mut self, result: &mut TestResult) {
        for i in 0..self.tests.len() {
            self.tests[i].run(result);
        }
    }
}

mod test_case_test {
    use super::*;

    pub fn test_template_method() {
        let mut test = TestCase::new(TestMethod);
        let mut result = TestResult::new();
        test.run(&mut result);
        assert_eq!("setup test_method teardown ", test.log);
    }

    pub fn test_result() {
        let mut test = TestCase::new(TestMethod);
        let mut result = TestResult::new();
        test.run(&mut result);
        assert_eq!("1 run, 0 failed", result.summary());
    }

    pub fn test_failed_result() {
        let mut test = TestCase::new(TestBrokenMethod);
        let mut result = TestResult::new();
        test.run(&mut result);
        assert_eq!("1 run, 1 failed", result.summary());
    }

    pub fn test_failed_result_formatting() {
        let mut result = TestResult::new();
        result.test_started();
        result.test_failed();
        assert_eq!("1 run, 1 failed", result.summary());
    }

    pub fn test_suite() {
        let mut suite = TestSuite::new();
        suite.add(TestCase::new(TestMethod));
        suite.add(TestCase::new(TestBrokenMethod));
        let mut result = TestResult::new();
        suite.run(&mut result);
        assert_eq!("2 run, 1 failed", result.summary());
    }
}

#[cfg(test)]
mod tests {
    use crate::xunit::*;

    #[test]
    fn method() {
        test_case_test::test_template_method();
        test_case_test::test_result();
        test_case_test::test_failed_result();
        test_case_test::test_failed_result_formatting();
        test_case_test::test_suite();
    }
}
