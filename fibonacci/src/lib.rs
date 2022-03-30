fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn fib_test() {
        let cases = vec![(0, 0), (1, 1), (2, 1), (3, 2)];
        for v in cases.iter() {
            assert_eq!(v.1, fib(v.0));
        }
    }
}
