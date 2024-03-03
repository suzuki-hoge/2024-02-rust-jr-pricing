use std::ops::{Add, Mul, Sub};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Amount {
    pub value: u64,
}

impl Add<Amount> for Amount {
    type Output = Amount;

    fn add(self, rhs: Amount) -> Self::Output {
        Amount { value: self.value + rhs.value }
    }
}

impl Sub<Amount> for Amount {
    type Output = Amount;

    fn sub(self, rhs: Amount) -> Self::Output {
        Amount { value: self.value - rhs.value }
    }
}

impl Mul<f32> for Amount {
    type Output = Amount;

    fn mul(self, rhs: f32) -> Self::Output {
        let value = (self.value as f32 * rhs) as u64;
        Amount { value: value / 10 * 10 }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::fundamental::amount::Amount;

    #[rstest]
    #[case(150, 20, 170)]
    fn add(#[case] lhs: u64, #[case] rhs: u64, #[case] exp: u64) {
        let amount1 = Amount { value: lhs };
        let amount2 = Amount { value: rhs };
        assert_eq!(Amount { value: exp }, amount1 + amount2);
    }

    #[rstest]
    #[case(150, 20, 130)]
    fn sub(#[case] lhs: u64, #[case] rhs: u64, #[case] exp: u64) {
        let amount1 = Amount { value: lhs };
        let amount2 = Amount { value: rhs };
        assert_eq!(Amount { value: exp }, amount1 - amount2);
    }

    #[rstest]
    #[case(150, 2.0, 300)]
    #[case(160, 0.5, 80)]
    #[case(150, 0.5, 70)]
    #[case(10, 0.5, 0)]
    #[case(10010, 0.9, 9000)]
    fn mul(#[case] lhs: u64, #[case] rhs: f32, #[case] exp: u64) {
        let amount = Amount { value: lhs };
        assert_eq!(Amount { value: exp }, amount * rhs);
    }
}
