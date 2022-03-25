#[cfg(test)]
mod tests {
    use crate::dollar::Dollar;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        let product = five.times(2);
        assert_eq!(10, product.amount());
        let product = five.times(3);
        assert_eq!(15, product.amount());
    }

    #[test]
    fn test_equality() {
        assert!(Dollar::new(5) == Dollar::new(5));
        assert_ne!(Dollar::new(5), Dollar::new(6));
    }
}
