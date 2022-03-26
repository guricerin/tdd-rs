use std::{collections::HashMap, ops::Add};

#[derive(Debug, PartialEq, Clone)]
enum Expression {
    Money(Money),
    Sum(Sum),
}

impl Add for Expression {
    type Output = Expression;

    fn add(self, other: Self) -> Self::Output {
        let sum = Sum::new(self, other);
        Self::Output::Sum(sum)
    }
}

impl Expression {
    fn amount(&self) -> i32 {
        match self {
            Expression::Money(m) => m.amount,
            Expression::Sum(s) => s.augend.amount() + s.addend.amount(),
        }
    }

    fn reduce(&self, bank: &Bank, to: &'static str) -> Expression {
        match self {
            Expression::Money(money) => {
                let rate = bank.rate(money.currency, to);
                let m = Money::new(money.amount / rate, to);
                Expression::Money(m)
            }
            Expression::Sum(sum) => {
                let l = sum.augend.reduce(bank, to);
                let r = sum.addend.reduce(bank, to);
                let m = Money::new(l.amount() + r.amount(), to);
                Expression::Money(m)
            }
        }
    }

    fn times(&self, multiplier: i32) -> Self {
        match self {
            Expression::Money(m) => {
                let m = Money::new(m.amount * multiplier, m.currency);
                Expression::Money(m)
            }
            Expression::Sum(s) => {
                let s = Sum::new(s.augend.times(multiplier), s.addend.times(multiplier));
                Expression::Sum(s)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Money {
    amount: i32,
    currency: &'static str,
}

#[derive(Debug, PartialEq, Clone)]
struct Sum {
    /// 被加算数
    augend: Box<Expression>,
    /// 加数
    addend: Box<Expression>,
}

impl Money {
    fn new(amount: i32, currency: &'static str) -> Self {
        Self {
            amount: amount,
            currency: currency,
        }
    }

    fn currency(&self) -> &'static str {
        self.currency
    }

    fn dollar(amount: i32) -> Money {
        let m = Money::new(amount, "USD");
        m
    }

    fn franc(amount: i32) -> Money {
        let m = Money::new(amount, "CHF");
        m
    }
}

impl Sum {
    fn new(augend: Expression, addend: Expression) -> Self {
        Self {
            augend: Box::new(augend),
            addend: Box::new(addend),
        }
    }
}

impl Add for Money {
    type Output = Expression;

    fn add(self, other: Self) -> Self::Output {
        let sum = Sum::new(Expression::Money(self), Expression::Money(other));
        Self::Output::Sum(sum)
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Pair {
    from: &'static str,
    to: &'static str,
}

impl Pair {
    fn new(from: &'static str, to: &'static str) -> Self {
        Self { from, to }
    }
}

struct Bank {
    rates: HashMap<Pair, i32>,
}

impl Bank {
    fn new() -> Self {
        Self {
            rates: HashMap::new(),
        }
    }

    fn add_rate(&mut self, from: &'static str, to: &'static str, rate: i32) {
        self.rates.insert(Pair::new(from, to), rate);
    }

    fn rate(&self, from: &'static str, to: &'static str) -> i32 {
        match self.rates.get(&Pair::new(from, to)) {
            Some(rate) => *rate,
            None => 1,
        }
    }

    fn reduce(&self, source: Expression, to: &'static str) -> Expression {
        source.reduce(&self, to)
    }
}

#[cfg(test)]
mod tests {
    use crate::money::{Bank, Expression, Money, Sum};

    fn dollar(amount: i32) -> Expression {
        let m = Money::dollar(amount);
        Expression::Money(m)
    }

    fn franc(amount: i32) -> Expression {
        let m = Money::franc(amount);
        Expression::Money(m)
    }

    #[test]
    fn multiplication() {
        let five = dollar(5);
        assert_eq!(dollar(10), five.times(2));
        assert_eq!(dollar(15), five.times(3));
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
        assert_eq!(dollar(10), reduced);
    }

    #[test]
    fn plus_returns_sum() {
        let five = dollar(5);
        let result = five.clone() + five.clone();
        match result {
            Expression::Money(_) => unreachable!(),
            Expression::Sum(result) => {
                assert_eq!(five, *result.augend);
                assert_eq!(five, *result.addend);
            }
        }
    }

    #[test]
    fn reduce_sum() {
        let sum = Sum::new(dollar(3), dollar(4));
        let sum = Expression::Sum(sum);
        let bank = Bank::new();
        let result = bank.reduce(sum, "USD");
        assert_eq!(dollar(7), result);
    }

    #[test]
    fn reduce_money() {
        let bank = Bank::new();
        let result = bank.reduce(dollar(1), "USD");
        assert_eq!(dollar(1), result);
    }

    #[test]
    fn reduce_money_different_currency() {
        let mut bank = Bank::new();
        bank.add_rate("CHF", "USD", 2);
        let result = bank.reduce(franc(2), "USD");
        assert_eq!(dollar(1), result);
    }

    #[test]
    fn identity_rate() {
        assert_eq!(1, Bank::new().rate("USD", "USD"));
    }

    #[test]
    fn mixed_addition() {
        let five_bucks = dollar(5);
        let ten_francs = franc(10);
        let mut bank = Bank::new();
        bank.add_rate("CHF", "USD", 2);
        let result = bank.reduce(five_bucks + ten_francs, "USD");
        assert_eq!(dollar(10), result);
    }

    #[test]
    fn sum_plus_money() {
        let five_bucks = dollar(5);
        let ten_francs = franc(10);
        let mut bank = Bank::new();
        bank.add_rate("CHF", "USD", 2);
        let sum = Sum::new(five_bucks.clone(), ten_francs);
        let sum = Expression::Sum(sum);
        let result = bank.reduce(sum + five_bucks, "USD");
        assert_eq!(dollar(15), result);
    }

    #[test]
    fn sum_times() {
        let five_bucks = dollar(5);
        let ten_francs = franc(10);
        let mut bank = Bank::new();
        bank.add_rate("CHF", "USD", 2);
        let sum = Sum::new(five_bucks, ten_francs);
        let sum = Expression::Sum(sum);
        let result = bank.reduce(sum.times(2), "USD");
        assert_eq!(dollar(20), result);
    }
}
