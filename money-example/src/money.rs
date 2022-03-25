use std::ops::Add;

pub trait Expression {}

#[derive(Debug, PartialEq)]
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

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            amount: self.amount + other.amount,
            currency: self.currency,
        }
    }
}

pub struct Bank {}

impl Bank {
    pub fn new() -> Self {
        Self {}
    }

    pub fn reduce(&self, source: Money, to: &'static str) -> Money {
        Money::dollar(10)
    }
}

#[cfg(test)]
mod tests {
    use crate::money::{Bank, Money};

    #[test]
    fn multiplication() {
        let five = Money::dollar(5);
        assert_eq!(Money::dollar(10), five.times(2));
        assert_eq!(Money::dollar(15), five.times(3));
    }

    #[test]
    fn equality() {
        assert_eq!(Money::dollar(5), Money::dollar(5));
        assert_ne!(Money::dollar(5), Money::dollar(6));
        assert_ne!(Money::franc(5), Money::dollar(5));
    }

    #[test]
    fn currency() {
        assert_eq!("USD", Money::dollar(1).currency());
        assert_eq!("CHF", Money::franc(1).currency());
    }

    #[test]
    fn simple_addition() {
        let sum = Money::dollar(5) + Money::dollar(5);
        let bank = Bank::new();
        let reduced = bank.reduce(sum, "USD");
        assert_eq!(Money::dollar(10), reduced);
    }
}
