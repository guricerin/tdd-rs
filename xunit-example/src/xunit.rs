enum TestKind {
    TestMethod,
}

struct TestCase {
    was_run: bool,
    kind: TestKind,
}

impl TestCase {
    fn new(kind: TestKind) -> Self {
        Self {
            was_run: false,
            kind: kind,
        }
    }

    fn run(&mut self) {
        match self.kind {
            TestKind::TestMethod => self.test_method(),
        }
    }

    fn test_method(&mut self) {
        self.was_run = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::xunit::*;

    #[test]
    fn method() {
        let mut test = TestCase::new(TestKind::TestMethod);
        assert!(!(test.was_run));
        test.run();
        assert!(test.was_run);
    }
}
