use std::ops::Add;

pub trait Expression {}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Money {
    amount: i32,
    currency: &'static str,
}

pub struct Sum {
    /// 被加算数
    augend: Money,
    /// 加数
    addend: Money,
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

impl Sum {
    pub fn new(augend: Money, addend: Money) -> Self {
        Self {
            augend: augend,
            addend: addend,
        }
    }

    pub fn reduce(&self, to: &'static str) -> Money {
        let amount = self.augend.amount + self.addend.amount;
        Money::new(amount, to)
    }
}

impl Add for Money {
    type Output = Sum;

    fn add(self, other: Self) -> Self::Output {
        Self::Output::new(self, other)
    }
}

pub struct Bank {}

impl Bank {
    pub fn new() -> Self {
        Self {}
    }

    pub fn reduce(&self, source: Sum, to: &'static str) -> Money {
        source.reduce(to)
    }
}

#[cfg(test)]
mod tests {
    use crate::money::{Bank, Money, Sum};

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

    #[test]
    fn plus_returns_sum() {
        let five = Money::dollar(5);
        let result = five + five;
        assert_eq!(five, result.augend);
        assert_eq!(five, result.addend);
    }

    #[test]
    fn reduce_sum() {
        let sum = Sum::new(Money::dollar(3), Money::dollar(4));
        let bank = Bank::new();
        let result = bank.reduce(sum, "USD");
        assert_eq!(Money::dollar(7), result);
    }
}
