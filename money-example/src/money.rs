#[derive(PartialEq)]
pub struct Money {
    amount: i32,
    currency: &'static str,
}

impl Money {
    pub fn new(amount: i32, currency: &'static str) -> Self {
        Self {
            amount: amount,
            currency: currency,
        }
    }

    pub fn times(&self, multiplier: i32) -> Self {
        Self {
            amount: self.amount * multiplier,
            currency: self.currency,
        }
    }

    pub fn currency(&self) -> &'static str {
        self.currency
    }

    pub fn dollar(amount: i32) -> Money {
        Money::new(amount, "USD")
    }

    pub fn franc(amount: i32) -> Money {
        Money::new(amount, "CHF")
    }
}

#[cfg(test)]
mod tests {
    use crate::money::Money;

    #[test]
    fn test_multiplication() {
        let five = Money::dollar(5);
        assert!(Money::dollar(10) == five.times(2));
        assert!(Money::dollar(15) == five.times(3));
    }

    #[test]
    fn test_equality() {
        assert!(Money::dollar(5) == Money::dollar(5));
        assert!(Money::dollar(5) != Money::dollar(6));
        assert!(Money::franc(5) != Money::dollar(5));
    }

    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::dollar(1).currency());
        assert_eq!("CHF", Money::franc(1).currency());
    }
}
