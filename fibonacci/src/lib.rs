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
        let cases = vec![(0, 0), (1, 1), (2, 1)];
        for v in cases.iter() {
            assert_eq!(v.1, fib(v.0));
        }
    }
}
