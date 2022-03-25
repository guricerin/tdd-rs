use crate::money::Money;

#[derive(Debug, PartialEq)]
pub struct Franc {
    amount: i32,
}

impl Franc {
    pub fn new(amount: i32) -> Self {
        Self { amount: amount }
    }
}

impl Money for Franc {
    fn times(&self, multiplier: i32) -> Self {
        Self::new(self.amount * multiplier)
    }
}
