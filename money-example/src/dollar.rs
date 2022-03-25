#[derive(Debug, PartialEq)]
pub struct Dollar {
    amount: i32,
}

impl Dollar {
    pub fn new(amount: i32) -> Self {
        Self { amount: amount }
    }

    pub fn times(&self, multiplier: i32) -> Self {
        Self::new(self.amount * multiplier)
    }

    pub fn amount(&self) -> i32 {
        self.amount
    }
}
