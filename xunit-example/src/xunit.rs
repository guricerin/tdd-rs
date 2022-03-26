struct WasRun {
    was_run: bool,
}

impl WasRun {
    fn new(name: String) -> Self {
        Self { was_run: false }
    }

    fn run(&mut self) {
        self.test_method();
    }

    fn test_method(&mut self) {
        self.was_run = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::xunit::WasRun;

    #[test]
    fn method() {
        let mut test = WasRun::new("test_method".to_owned());
        assert!(!(test.was_run));
        test.run();
        assert!(test.was_run);
    }
}
