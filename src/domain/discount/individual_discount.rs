use crate::domain::base::departure_date::Season;
use crate::domain::base::number_of_passengers::NumberOfPassengers;
use crate::domain::base::ride_section::RideSection;
use crate::domain::discount::individual_discount::IndividualDiscount::{GroupDiscountUnder30, RoundTripDiscount};
use crate::domain::fare::express_fare::ExpressFare;
use crate::domain::fare::train_fare::TrainFare;

#[derive(PartialEq, Debug)]
pub enum IndividualDiscount {
    RoundTripDiscount,
    GroupDiscountUnder30 { discount_rate: f32 },
}

impl IndividualDiscount {
    pub fn apply(&self, fare: (TrainFare, ExpressFare)) -> (TrainFare, ExpressFare) {
        match self {
            RoundTripDiscount => (TrainFare { value: fare.0.value * 0.9 }, fare.1),
            GroupDiscountUnder30 { discount_rate } => (
                TrainFare { value: fare.0.value * *discount_rate },
                ExpressFare { value: fare.1.value * *discount_rate },
            ),
        }
    }
    // この作りだと (train * rate + express * rate) はできるが (train + express) * rate ができない
    // 仕様がはっきりしないので前者で濁すが、要再設計

    // 0.9 を 2 度適用した場合の切り捨てタイミングが不明瞭
    // 個別切り捨てだと 0.85 -> 0.9 と 0.9 -> 0.85 で結果が変わるため 0.765 を適用する方がよさそう
    // ほかにどのような割引を考慮するべきかわからないと汎用化できないため、暫定で個別適用とする
}

pub fn judge_individual_discounts(
    ride_section: &RideSection,
    number_of_passengers: &NumberOfPassengers,
    season: &Season,
) -> Vec<IndividualDiscount> {
    let mut discounts = vec![];

    if 601.0 <= ride_section.get_operation_kilometer().value {
        discounts.push(RoundTripDiscount)
    }
    if (8..=30).contains(&number_of_passengers.total()) {
        let discount_rate = match season {
            Season::Regular => 0.85,
            Season::OffPeak => 0.85,
            Season::Peak => 0.9,
        };
        discounts.push(GroupDiscountUnder30 { discount_rate });
    }

    discounts
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use crate::domain::base::departure_date::Season;
    use crate::domain::base::departure_date::Season::*;
    use crate::domain::base::number_of_passengers::NumberOfPassengers;
    use crate::domain::base::ride_section::Station::*;
    use crate::domain::base::ride_section::{RideSection, Station};
    use crate::domain::discount::individual_discount::IndividualDiscount::{GroupDiscountUnder30, RoundTripDiscount};
    use crate::domain::discount::individual_discount::{judge_individual_discounts, IndividualDiscount};
    use crate::domain::fare::express_fare::ExpressFare;
    use crate::domain::fare::train_fare::TrainFare;
    use crate::fundamental::amount::Amount;

    #[rstest]
    #[case(Tokyo, ShinOsaka, 1, 0, Regular)]
    #[case(Tokyo, ShinOsaka, 31, 0, Regular)]
    #[case(Tokyo, ShinOsaka, 16, 15, Regular)]
    fn test_create_individual_discounts_no_result(
        #[case] departure: Station,
        #[case] arrival: Station,
        #[case] adult: usize,
        #[case] child: usize,
        #[case] season: Season,
    ) {
        let ride_section = RideSection { departure, arrival };
        let number_of_passengers = NumberOfPassengers { adult, child };
        assert_eq!(0, judge_individual_discounts(&ride_section, &number_of_passengers, &season).len());
    }

    #[rstest]
    #[case(Tokyo, Himeji, 1, 0, Regular, RoundTripDiscount)]
    #[case(Tokyo, ShinOsaka, 8, 0, Peak, GroupDiscountUnder30 { discount_rate: 0.9 })]
    #[case(Tokyo, ShinOsaka, 8, 0, Regular, GroupDiscountUnder30 { discount_rate: 0.85 })]
    #[case(Tokyo, ShinOsaka, 8, 0, OffPeak, GroupDiscountUnder30 { discount_rate: 0.85 })]
    #[case(Tokyo, ShinOsaka, 4, 4, Regular, GroupDiscountUnder30 { discount_rate: 0.85 })]
    #[case(Tokyo, ShinOsaka, 15, 15, Regular, GroupDiscountUnder30 { discount_rate: 0.85 })]
    fn test_create_individual_discounts(
        #[case] departure: Station,
        #[case] arrival: Station,
        #[case] adult: usize,
        #[case] child: usize,
        #[case] season: Season,
        #[case] exp: IndividualDiscount,
    ) {
        let ride_section = RideSection { departure, arrival };
        let number_of_passengers = NumberOfPassengers { adult, child };
        let act = judge_individual_discounts(&ride_section, &number_of_passengers, &season);
        assert_eq!(1, act.len());
        assert_eq!(exp, act[0]);
    }

    #[rstest]
    #[case(RoundTripDiscount, 10010, 5920, 9000, 5920)]
    #[case(GroupDiscountUnder30 { discount_rate: 0.9 }, 8910, 5490, 8010, 4940)]
    #[case(GroupDiscountUnder30 { discount_rate: 0.85 }, 8910, 5490, 7570, 4660)]
    fn apply(
        #[case] sut: IndividualDiscount,
        #[case] train_fare: u64,
        #[case] express_fare: u64,
        #[case] applied_train_fare: u64,
        #[case] applied_express_fare: u64,
    ) {
        let fare =
            (TrainFare { value: Amount { value: train_fare } }, ExpressFare { value: Amount { value: express_fare } });
        let exp = (
            TrainFare { value: Amount { value: applied_train_fare } },
            ExpressFare { value: Amount { value: applied_express_fare } },
        );
        assert_eq!(exp, sut.apply(fare));
    }
}
