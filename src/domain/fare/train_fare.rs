use crate::domain::base::ride_section::{RideSection, Station};

use crate::fundamental::amount::Amount;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct TrainFare {
    pub value: Amount,
}

pub fn calc_train_fare(ride_section: &RideSection) -> TrainFare {
    match ride_section.get_station_pair() {
        (Station::Tokyo, Station::ShinOsaka) => TrainFare { value: Amount { value: 8910 } },
        (Station::Tokyo, Station::Himeji) => TrainFare { value: Amount { value: 10010 } },
        _ => panic!("unexpected ride section"),
    }
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use crate::domain::base::ride_section::Station::*;
    use crate::domain::base::ride_section::{RideSection, Station};

    use crate::domain::fare::train_fare::{calc_train_fare, TrainFare};
    use crate::fundamental::amount::Amount;

    #[rstest]
    #[case(Tokyo, ShinOsaka, 8910)]
    #[case(ShinOsaka, Tokyo, 8910)]
    #[case(Tokyo, Himeji, 10010)]
    #[case(Himeji, Tokyo, 10010)]
    fn test_create_train_fare(#[case] departure: Station, #[case] arrival: Station, #[case] exp: u64) {
        let ride_section = RideSection { departure, arrival };
        assert_eq!(TrainFare { value: Amount { value: exp } }, calc_train_fare(&ride_section));
    }
}
