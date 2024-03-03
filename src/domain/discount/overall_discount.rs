use std::cmp::max;

use crate::domain::base::number_of_passengers::NumberOfPassengers;
use crate::domain::discount::overall_discount::OverallDiscount::GroupDiscountMore31;

#[derive(PartialEq, Debug)]
pub enum OverallDiscount {
    GroupDiscountMore31 { free_count: usize },
}

pub fn judge_overall_discount(number_of_passengers: &NumberOfPassengers) -> Option<OverallDiscount> {
    if 31 <= number_of_passengers.total() {
        let free_count = max(number_of_passengers.total() / 50, 1);
        Some(GroupDiscountMore31 { free_count })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::domain::base::number_of_passengers::NumberOfPassengers;
    use crate::domain::discount::overall_discount::OverallDiscount::*;
    use crate::domain::discount::overall_discount::{judge_overall_discount, OverallDiscount};

    #[rstest]
    #[case(1, 0)]
    #[case(0, 1)]
    #[case(7, 0)]
    #[case(0, 7)]
    fn test_create_overall_discounts_no_result(#[case] adult: usize, #[case] child: usize) {
        let number_of_passengers = NumberOfPassengers { adult, child };
        assert_eq!(None, judge_overall_discount(&number_of_passengers));
    }

    #[rstest]
    #[case(31, 0, GroupDiscountMore31 { free_count: 1 })]
    #[case(50, 0, GroupDiscountMore31 { free_count: 1 })]
    #[case(51, 0, GroupDiscountMore31 { free_count: 1 })]
    #[case(99, 0, GroupDiscountMore31 { free_count: 1 })]
    #[case(100, 0, GroupDiscountMore31 { free_count: 2 })]
    fn test_create_overall_discounts(#[case] adult: usize, #[case] child: usize, #[case] exp: OverallDiscount) {
        let number_of_passengers = NumberOfPassengers { adult, child };
        assert_eq!(exp, judge_overall_discount(&number_of_passengers).unwrap());
    }
}
