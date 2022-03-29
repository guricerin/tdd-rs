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

struct TestCase {
    log: String,
}

impl TestCase {
    fn new() -> Self {
        Self { log: "".to_owned() }
    }

    fn setup(&mut self) {
        self.log = "setup ".to_owned();
    }

    fn run<F>(&mut self, f: F) -> TestResult
    where
        F: FnOnce(&mut TestCase) -> Result<()>,
    {
        let mut result = TestResult::new();
        result.test_started();
        self.setup();
        match f(self) {
            Ok(_) => (),
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

struct TestCaseTest {}

impl TestCaseTest {
    fn new() -> Self {
        Self {}
    }

    fn test_template_method(&self) {
        let mut test = TestCase::new();
        test.run(TestCase::test_method);
        assert_eq!("setup test_method teardown ", test.log);
    }

    fn test_result(&self) {
        let mut test = TestCase::new();
        let result = test.run(TestCase::test_method);
        assert_eq!("1 run, 0 failed", result.summary());
    }

    fn test_failed_result(&self) {
        let mut test = TestCase::new();
        let result = test.run(TestCase::test_broken_method);
        assert_eq!("1 run, 1 failed", result.summary());
    }

    fn test_failed_result_formatting(&self) {
        let mut result = TestResult::new();
        result.test_started();
        result.test_failed();
        assert_eq!("1 run, 1 failed", result.summary());
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
    }
}
