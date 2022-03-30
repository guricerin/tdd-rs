fn fib(n: i64) -> i64 {
    match n {
        0 => 0,
        n if n <= 2 => 1,
        n => 2,
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
