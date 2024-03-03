use ReserveType::{RoundTrip, SingleTrip};

use crate::domain::base::number_of_passengers::NumberOfPassengers;
use crate::domain::base::reserve_type::ReserveType;

use crate::domain::discount::individual_discount::IndividualDiscount;
use crate::domain::discount::overall_discount::OverallDiscount;
use crate::domain::discount::overall_discount::OverallDiscount::GroupDiscountMore31;

use crate::domain::fare::single_trip_adult_fare::SingleTripAdultFare;

use crate::fundamental::amount::Amount;

pub fn calc_total_fare(
    reserve_type: &ReserveType,
    number_of_passengers: &NumberOfPassengers,
    single_trip_adult_fare: &SingleTripAdultFare,
    overall_discount: &Option<OverallDiscount>,
    individual_discounts: &[IndividualDiscount],
) -> Amount {
    let (total_adult_train_free, total_adult_express_fee) = {
        let (discounted_train_fare, discounted_express_fare) = individual_discounts
            .iter()
            .fold(single_trip_adult_fare.get_adult_fare(), |acc, discount| discount.apply(acc));

        let count = match overall_discount {
            Some(GroupDiscountMore31 { free_count }) => number_of_passengers.adult - free_count,
            _ => number_of_passengers.adult,
        } as f32;

        (discounted_train_fare.value * count, discounted_express_fare.value * count)
    };

    let (total_child_train_free, total_child_express_fee) = {
        let (discounted_train_fare, discounted_express_fare) = individual_discounts
            .iter()
            .fold(single_trip_adult_fare.get_child_fare(), |acc, discount| discount.apply(acc));

        let count = number_of_passengers.child as f32;

        (discounted_train_fare.value * count, discounted_express_fare.value * count)
    };

    let way = match reserve_type {
        SingleTrip => 1.0,
        RoundTrip => 2.0,
    };

    (total_adult_train_free + total_adult_express_fee + total_child_train_free + total_child_express_fee) * way
}

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
mod tests {
    use rstest::rstest;

    use IndividualDiscount::RoundTripDiscount;

    use crate::domain::base::number_of_passengers::NumberOfPassengers;
    use crate::domain::base::reserve_type::ReserveType;
    use crate::domain::base::reserve_type::ReserveType::*;
    use crate::domain::discount::individual_discount::IndividualDiscount;
    use crate::domain::discount::individual_discount::IndividualDiscount::GroupDiscountUnder30;
    use crate::domain::discount::overall_discount::OverallDiscount;
    use crate::domain::discount::overall_discount::OverallDiscount::GroupDiscountMore31;
    use crate::domain::fare::express_fare::ExpressFare;
    use crate::domain::fare::single_trip_adult_fare::SingleTripAdultFare;
    use crate::domain::fare::total_fare::calc_total_fare;
    use crate::domain::fare::train_fare::TrainFare;
    use crate::fundamental::amount::Amount;

    #[rstest]
    // おとな 1 人、こども 0 人
    #[case(SingleTrip, 1, 0, 8910, 5490, None, None, None, 14400)]
    // おとな 1 人、こども 1 人
    #[case(SingleTrip, 1, 1, 8910, 5490, None, None, None, 21590)]
    // おとな 2 人、こども 2 人
    #[case(SingleTrip, 2, 2, 8910, 5490, None, None, None, 43180)]
    // 往復
    #[case(RoundTrip, 1, 0, 8910, 5490, None, None, None, 28800)]
    // 個別割引 - 往復割引
    #[case(RoundTrip, 1, 0, 10010, 5920, None, Some(RoundTripDiscount), None, 29840)]
    // 個別割引 - 団体割引
    #[case(SingleTrip, 10, 0, 8910, 5490, None, None, Some(GroupDiscountUnder30 { discount_rate: 0.9 }), 129500)]
    // 個別割引 - 往復割引 & 団体割引
    #[case(RoundTrip, 10, 0, 10010, 5920, None, Some(RoundTripDiscount), Some(GroupDiscountUnder30 { discount_rate: 0.85 }), 253600)]
    // 全体割引 - 団体割引
    #[case(SingleTrip, 50, 0, 8910, 5490, Some(GroupDiscountMore31 {free_count: 1}), None, None, 705600)]
    // 個別割引 - 往復割引 & 全体割引 - 団体割引
    #[case(RoundTrip, 100, 0, 10010, 5920, Some(GroupDiscountMore31 {free_count: 2}), Some(RoundTripDiscount), None, 2924320)]
    fn test_calc_total_fare(
        #[case] reserve_type: ReserveType,
        #[case] adult: usize,
        #[case] child: usize,
        #[case] train_fare: u64,
        #[case] express_fare: u64,
        #[case] overall_discount: Option<OverallDiscount>,
        #[case] individual_discount1: Option<IndividualDiscount>,
        #[case] individual_discount2: Option<IndividualDiscount>,
        #[case] exp: u64,
    ) {
        let number_of_passengers = NumberOfPassengers { adult, child };
        let single_trip_adult_fare = SingleTripAdultFare {
            train_fare: TrainFare { value: Amount { value: train_fare } },
            express_fare: ExpressFare { value: Amount { value: express_fare } },
        };
        assert_eq!(
            Amount { value: exp },
            calc_total_fare(
                &reserve_type,
                &number_of_passengers,
                &single_trip_adult_fare,
                &overall_discount,
                &match (individual_discount1, individual_discount2) {
                    (Some(discount1), Some(discount2)) => vec![discount1, discount2],
                    (Some(discount1), None) => vec![discount1],
                    (None, Some(discount2)) => vec![discount2],
                    (None, None) => vec![],
                },
            )
        );
    }
}
