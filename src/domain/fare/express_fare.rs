use crate::domain::base::departure_date::{DepartureDate, Season};
use crate::domain::base::ride_section::{RideSection, Station};
use crate::domain::base::seat_type::SeatType;
use crate::domain::base::train::Train;
use crate::fundamental::amount::Amount;

#[derive(Eq, PartialEq, Debug)]
pub struct ExpressFare {
    pub value: Amount,
}

pub fn create_express_fare(
    train: &Train,
    seat_type: &SeatType,
    ride_section: &RideSection,
    departure_date: &DepartureDate,
) -> ExpressFare {
    fn create_reserved_hikari_express_fare(ride_section: &RideSection) -> Amount {
        match ride_section.get_station_pair() {
            (Station::Tokyo, Station::ShinOsaka) => Amount { value: 5490 },
            (Station::Tokyo, Station::Himeji) => Amount { value: 5920 },
            _ => panic!("unexpected ride section"),
        }
    }

    fn create_reserved_nozomi_express_fare(ride_section: &RideSection) -> Amount {
        let hikari = create_reserved_hikari_express_fare(ride_section);
        let addition = match ride_section.get_station_pair() {
            (Station::Tokyo, Station::ShinOsaka) => Amount { value: 320 },
            (Station::Tokyo, Station::Himeji) => Amount { value: 530 },
            _ => panic!("unexpected ride section"),
        };
        hikari + addition
    }

    fn create_free_express_fare(ride_section: &RideSection) -> Amount {
        let hikari = create_reserved_hikari_express_fare(ride_section);
        let subtraction = Amount { value: 530 };
        hikari - subtraction
    }

    let amount = match (seat_type, train) {
        (SeatType::Reserved, Train::Hikari) => create_reserved_hikari_express_fare(ride_section),
        (SeatType::Reserved, Train::Nozomi) => create_reserved_nozomi_express_fare(ride_section),
        (SeatType::Free, _) => create_free_express_fare(ride_section),
    };

    ExpressFare {
        value: match (seat_type, departure_date.get_season()) {
            (SeatType::Reserved, Season::Regular) => amount,
            (SeatType::Reserved, Season::OffPeak) => amount - Amount { value: 200 },
            (SeatType::Reserved, Season::Peak) => amount + Amount { value: 200 },
            (SeatType::Free, _) => amount,
        },
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use rstest::rstest;

    use crate::domain::base::departure_date::DepartureDate;
    use crate::domain::base::ride_section::Station::*;
    use crate::domain::base::ride_section::{RideSection, Station};
    use crate::domain::base::seat_type::SeatType;
    use crate::domain::base::seat_type::SeatType::*;
    use crate::domain::base::train::Train;
    use crate::domain::base::train::Train::*;
    use crate::domain::fare::express_fare::{create_express_fare, ExpressFare};
    use crate::fundamental::amount::Amount;

    #[rstest]
    // ひかり指定席
    #[case(Hikari, Reserved, Tokyo, ShinOsaka, 12, 01, 5490)]
    #[case(Hikari, Reserved, ShinOsaka, Tokyo, 12, 01, 5490)]
    #[case(Hikari, Reserved, Tokyo, ShinOsaka, 01, 20, 5290)]
    #[case(Hikari, Reserved, ShinOsaka, Tokyo, 01, 20, 5290)]
    #[case(Hikari, Reserved, Tokyo, ShinOsaka, 01, 05, 5690)]
    #[case(Hikari, Reserved, ShinOsaka, Tokyo, 01, 05, 5690)]
    // ひかり指定席
    #[case(Hikari, Reserved, Tokyo, Himeji, 12, 01, 5920)]
    #[case(Hikari, Reserved, Himeji, Tokyo, 12, 01, 5920)]
    #[case(Hikari, Reserved, Tokyo, Himeji, 01, 20, 5720)]
    #[case(Hikari, Reserved, Himeji, Tokyo, 01, 20, 5720)]
    #[case(Hikari, Reserved, Tokyo, Himeji, 01, 05, 6120)]
    #[case(Hikari, Reserved, Himeji, Tokyo, 01, 05, 6120)]
    // のぞみ指定席
    #[case(Nozomi, Reserved, Tokyo, ShinOsaka, 12, 01, 5810)]
    #[case(Nozomi, Reserved, ShinOsaka, Tokyo, 12, 01, 5810)]
    #[case(Nozomi, Reserved, Tokyo, ShinOsaka, 01, 20, 5610)]
    #[case(Nozomi, Reserved, ShinOsaka, Tokyo, 01, 20, 5610)]
    #[case(Nozomi, Reserved, Tokyo, ShinOsaka, 01, 05, 6010)]
    #[case(Nozomi, Reserved, ShinOsaka, Tokyo, 01, 05, 6010)]
    // のぞみ指定席
    #[case(Nozomi, Reserved, Tokyo, Himeji, 12, 01, 6450)]
    #[case(Nozomi, Reserved, Himeji, Tokyo, 12, 01, 6450)]
    #[case(Nozomi, Reserved, Tokyo, Himeji, 01, 20, 6250)]
    #[case(Nozomi, Reserved, Himeji, Tokyo, 01, 20, 6250)]
    #[case(Nozomi, Reserved, Tokyo, Himeji, 01, 05, 6650)]
    #[case(Nozomi, Reserved, Himeji, Tokyo, 01, 05, 6650)]
    // ひかり自由席
    #[case(Hikari, Free, Tokyo, ShinOsaka, 12, 01, 4960)]
    #[case(Hikari, Free, ShinOsaka, Tokyo, 12, 01, 4960)]
    #[case(Hikari, Free, Tokyo, ShinOsaka, 01, 20, 4960)]
    #[case(Hikari, Free, ShinOsaka, Tokyo, 01, 20, 4960)]
    #[case(Hikari, Free, Tokyo, ShinOsaka, 01, 05, 4960)]
    #[case(Hikari, Free, ShinOsaka, Tokyo, 01, 05, 4960)]
    // ひかり自由席
    #[case(Hikari, Free, Tokyo, Himeji, 12, 01, 5390)]
    #[case(Hikari, Free, Himeji, Tokyo, 12, 01, 5390)]
    #[case(Hikari, Free, Tokyo, Himeji, 01, 20, 5390)]
    #[case(Hikari, Free, Himeji, Tokyo, 01, 20, 5390)]
    #[case(Hikari, Free, Tokyo, Himeji, 01, 05, 5390)]
    #[case(Hikari, Free, Himeji, Tokyo, 01, 05, 5390)]
    // のぞみ自由席
    #[case(Nozomi, Free, Tokyo, ShinOsaka, 12, 01, 4960)]
    #[case(Nozomi, Free, ShinOsaka, Tokyo, 12, 01, 4960)]
    #[case(Nozomi, Free, Tokyo, ShinOsaka, 01, 20, 4960)]
    #[case(Nozomi, Free, ShinOsaka, Tokyo, 01, 20, 4960)]
    #[case(Nozomi, Free, Tokyo, ShinOsaka, 01, 05, 4960)]
    #[case(Nozomi, Free, ShinOsaka, Tokyo, 01, 05, 4960)]
    // のぞみ自由席
    #[case(Nozomi, Free, Tokyo, Himeji, 12, 01, 5390)]
    #[case(Nozomi, Free, Himeji, Tokyo, 12, 01, 5390)]
    #[case(Nozomi, Free, Tokyo, Himeji, 01, 20, 5390)]
    #[case(Nozomi, Free, Himeji, Tokyo, 01, 20, 5390)]
    #[case(Nozomi, Free, Tokyo, Himeji, 01, 05, 5390)]
    #[case(Nozomi, Free, Himeji, Tokyo, 01, 05, 5390)]
    fn test_create_express_fare(
        #[case] train: Train,
        #[case] seat_type: SeatType,
        #[case] departure: Station,
        #[case] arrival: Station,
        #[case] m: u32,
        #[case] d: u32,
        #[case] exp: u64,
    ) {
        let ride_section = RideSection { departure, arrival };
        let departure_date = DepartureDate { value: NaiveDate::from_ymd_opt(2024, m, d).unwrap() };
        assert_eq!(
            ExpressFare { value: Amount { value: exp } },
            create_express_fare(&train, &seat_type, &ride_section, &departure_date)
        );
    }
}
