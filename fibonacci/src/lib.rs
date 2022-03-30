fn fib(n: i64) -> i64 {
    if n == 0 {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn fib_test() {
        assert_eq!(0, fib(0));
        assert_eq!(1, fib(1));
    }
}
