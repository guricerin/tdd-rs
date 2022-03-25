use crate::{dollar::Dollar, franc::Franc};

pub trait Money {
    fn times(&self, multiplier: i32) -> Self;
}

struct MoneyFactory;

impl MoneyFactory {
    pub fn dollar(amount: i32) -> Dollar {
        Dollar::new(amount)
    }

    pub fn franc(amount: i32) -> Franc {
        Franc::new(amount)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dollar::Dollar,
        franc::Franc,
        money::{Money, MoneyFactory},
    };

    #[test]
    fn test_multiplication() {
        let five = MoneyFactory::dollar(5);
        assert_eq!(MoneyFactory::dollar(10), five.times(2));
        assert_eq!(MoneyFactory::dollar(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert!(MoneyFactory::dollar(5) == MoneyFactory::dollar(5));
        assert_ne!(MoneyFactory::dollar(5), MoneyFactory::dollar(6));
        assert!(MoneyFactory::franc(5) == MoneyFactory::franc(5));
        assert_ne!(MoneyFactory::franc(5), MoneyFactory::franc(6));
        // assert_ne!(Franc::new(5), MoneyFactory::dollar(6));
    }

    #[test]
    fn test_franc_multiplication() {
        let five = MoneyFactory::franc(5);
        assert_eq!(MoneyFactory::franc(10), five.times(2));
        assert_eq!(MoneyFactory::franc(15), five.times(3));
    }
}
