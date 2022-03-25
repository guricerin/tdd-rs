pub struct Dollar {
    amount: i32,
}

impl Dollar {
    pub fn new(amount: i32) -> Self {
        Self { amount: 10 }
    }

    pub fn times(&mut self, multiplier: i32) {}

    pub fn amount(&self) -> i32 {
        self.amount
    }
}
