use crate::money::Money;

#[derive(Debug, PartialEq)]
pub struct Dollar {
    amount: i32,
}

impl Dollar {
    pub fn new(amount: i32) -> Self {
        Self { amount: amount }
    }
}

impl Money for Dollar {
    fn times(&self, multiplier: i32) -> Self {
        Self::new(self.amount * multiplier)
    }
}
