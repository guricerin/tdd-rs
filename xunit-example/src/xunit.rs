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

    fn run<F>(&mut self, f: F)
    where
        F: FnOnce(&mut TestCase),
    {
        self.setup();
        f(self);
        self.teardown();
    }

    fn teardown(&mut self) {
        self.log = format!("{}teardown ", self.log);
    }

    fn test_method(&mut self) {
        self.log = format!("{}test_method ", self.log);
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
}

#[cfg(test)]
mod tests {
    use crate::xunit::*;

    #[test]
    fn method() {
        let test = TestCaseTest::new();
        test.test_template_method();
    }
}
