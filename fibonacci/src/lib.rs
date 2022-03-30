fn fib(n: i64) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn fib_test() {
        assert_eq!(0, fib(0));
    }
}
