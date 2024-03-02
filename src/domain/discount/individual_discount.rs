use crate::domain::base::ride_section::RideSection;
use crate::domain::discount::individual_discount::IndividualDiscount::RoundTripDiscount;
use crate::domain::fare::express_fare::ExpressFare;
use crate::domain::fare::train_fare::TrainFare;

#[derive(Eq, PartialEq, Debug)]
pub enum IndividualDiscount {
    RoundTripDiscount,
}

impl IndividualDiscount {
    fn apply(&self, train_fare: TrainFare, express_fare: ExpressFare) -> (TrainFare, ExpressFare) {
        match self {
            RoundTripDiscount => (TrainFare { value: train_fare.value * 0.9 }, express_fare),
        }
    }
}

pub fn create_individual_discounts(ride_section: &RideSection) -> Vec<IndividualDiscount> {
    if 601.0 <= ride_section.get_operation_kilometer().value {
        vec![RoundTripDiscount]
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::domain::base::ride_section::Station::*;
    use crate::domain::base::ride_section::{RideSection, Station};
    use crate::domain::discount::individual_discount::IndividualDiscount::RoundTripDiscount;
    use crate::domain::discount::individual_discount::{create_individual_discounts, IndividualDiscount};
    use crate::domain::fare::express_fare::ExpressFare;
    use crate::domain::fare::train_fare::TrainFare;
    use crate::fundamental::amount::Amount;

    #[rstest]
    #[case(Tokyo, ShinOsaka)]
    fn test_create_individual_discounts_no_result(#[case] departure: Station, #[case] arrival: Station) {
        let ride_section = RideSection { departure, arrival };
        assert_eq!(0, create_individual_discounts(&ride_section).len());
    }

    #[rstest]
    #[case(Tokyo, Himeji, RoundTripDiscount)]
    fn test_create_individual_discounts(
        #[case] departure: Station,
        #[case] arrival: Station,
        #[case] exp: IndividualDiscount,
    ) {
        let ride_section = RideSection { departure, arrival };
        let act = create_individual_discounts(&ride_section);
        assert_eq!(1, act.len());
        assert_eq!(exp, act[0]);
    }

    #[rstest]
    #[case(10010, 5920, 9000, 5920)]
    fn round_trip_discount(
        #[case] train_fare: u64,
        #[case] express_fare: u64,
        #[case] applied_train_fare: u64,
        #[case] applied_express_fare: u64,
    ) {
        let train_fare = TrainFare { value: Amount { value: train_fare } };
        let express_fare = ExpressFare { value: Amount { value: express_fare } };
        let exp = (
            TrainFare { value: Amount { value: applied_train_fare } },
            ExpressFare { value: Amount { value: applied_express_fare } },
        );
        assert_eq!(exp, RoundTripDiscount.apply(train_fare, express_fare));
    }
}
