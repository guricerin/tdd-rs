enum TestKind {
    TestMethod,
}

struct TestCase {
    was_setup: bool,
    was_run: bool,
    kind: TestKind,
}

impl TestCase {
    fn new(kind: TestKind) -> Self {
        Self {
            was_setup: true,
            was_run: false,
            kind: kind,
        }
    }

    fn setup(&mut self) {
        self.was_setup = true;
        self.was_run = false;
    }

    fn run(&mut self) {
        self.setup();
        match self.kind {
            TestKind::TestMethod => self.test_method(),
        }
    }

    fn test_method(&mut self) {
        self.was_run = true;
    }
}

struct TestCaseTest {
    test: TestCase,
}

impl TestCaseTest {
    fn setup(kind: TestKind) -> Self {
        Self {
            test: TestCase::new(kind),
        }
    }

    fn test_running(&mut self) {
        self.test.run();
        assert!(self.test.was_run);
    }

    fn test_setup(&mut self) {
        self.test.run();
        assert!(self.test.was_setup);
    }
}

#[cfg(test)]
mod tests {
    use crate::xunit::*;

    #[test]
    fn method() {
        let mut test = TestCaseTest::setup(TestKind::TestMethod);
        test.test_running();
        test.test_setup();
    }
}
