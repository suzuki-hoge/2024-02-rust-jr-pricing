use crate::domain::base::departure_date::Season;
use crate::domain::base::number_of_passengers::NumberOfPassengers;
use crate::domain::discount::overall_discount::OverallDiscount::{GroupDiscountMore31, GroupDiscountUnder30};
use std::cmp::max;

#[derive(PartialEq, Debug)]
pub enum OverallDiscount {
    GroupDiscountUnder30 { discount_rate: f32 },
    GroupDiscountMore31 { free_count: usize },
}

pub fn create_overall_discounts(number_of_passengers: &NumberOfPassengers, season: &Season) -> Vec<OverallDiscount> {
    let total = number_of_passengers.total();

    if (8..=30).contains(&total) {
        let discount_rate = match season {
            Season::Regular => 0.85,
            Season::OffPeak => 0.85,
            Season::Peak => 0.9,
        };
        vec![GroupDiscountUnder30 { discount_rate }]
    } else if 31 <= total {
        let free_count = max(number_of_passengers.total() / 50, 1);
        vec![GroupDiscountMore31 { free_count }]
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::domain::base::departure_date::Season;
    use crate::domain::base::departure_date::Season::*;
    use crate::domain::base::number_of_passengers::NumberOfPassengers;
    use crate::domain::discount::overall_discount::OverallDiscount::*;
    use crate::domain::discount::overall_discount::{create_overall_discounts, OverallDiscount};

    #[rstest]
    #[case(1, 0, Regular)]
    #[case(0, 1, Regular)]
    #[case(7, 0, Regular)]
    #[case(0, 7, Regular)]
    fn test_create_overall_discounts_no_result(#[case] adult: usize, #[case] child: usize, #[case] season: Season) {
        let number_of_passengers = NumberOfPassengers { adult, child };
        assert_eq!(0, create_overall_discounts(&number_of_passengers, &season).len());
    }

    #[rstest]
    #[case(8, 0, Regular, GroupDiscountUnder30 { discount_rate: 0.85 })]
    #[case(8, 0, OffPeak, GroupDiscountUnder30 { discount_rate: 0.85 })]
    #[case(8, 0, Peak, GroupDiscountUnder30 { discount_rate: 0.9 })]
    #[case(30, 0, Regular, GroupDiscountUnder30 { discount_rate: 0.85 })]
    #[case(30, 0, OffPeak, GroupDiscountUnder30 { discount_rate: 0.85 })]
    #[case(30, 0, Peak, GroupDiscountUnder30 { discount_rate: 0.9 })]
    #[case(31, 0, Regular, GroupDiscountMore31 { free_count: 1 })]
    #[case(50, 0, Regular, GroupDiscountMore31 { free_count: 1 })]
    #[case(51, 0, Regular, GroupDiscountMore31 { free_count: 1 })]
    #[case(99, 0, Regular, GroupDiscountMore31 { free_count: 1 })]
    #[case(100, 0, Regular, GroupDiscountMore31 { free_count: 2 })]
    fn test_create_overall_discounts(
        #[case] adult: usize,
        #[case] child: usize,
        #[case] season: Season,
        #[case] exp: OverallDiscount,
    ) {
        let number_of_passengers = NumberOfPassengers { adult, child };
        let act = create_overall_discounts(&number_of_passengers, &season);
        assert_eq!(1, act.len());
        assert_eq!(exp, act[0]);
    }
}
