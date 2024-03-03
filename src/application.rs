use chrono::NaiveDate;

use crate::domain::base::departure_date::DepartureDate;
use crate::domain::base::number_of_passengers::NumberOfPassengers;
use crate::domain::base::reserve_type::ReserveType;
use crate::domain::base::ride_section::{RideSection, Station};
use crate::domain::base::seat_type::SeatType;
use crate::domain::base::train::Train;
use crate::domain::discount::individual_discount::judge_individual_discounts;
use crate::domain::discount::overall_discount::judge_overall_discount;
use crate::domain::fare::single_trip_adult_fare::calc_single_trip_adult_fare;
use crate::domain::fare::total_fare::calc_total_fare;

#[allow(clippy::too_many_arguments)]
pub fn invoke(
    departure_input: StationInput,
    arrival_input: StationInput,
    train_input: TrainInput,
    seat_type_input: SeatTypeInput,
    reserve_type_input: ReserveTypeInput,
    y: i32,
    m: u32,
    d: u32,
    adult: usize,
    child: usize,
) -> u64 {
    // バリデーションされたパラメータをドメインに変換する
    let ride_section = RideSection { departure: departure_input.as_domain(), arrival: arrival_input.as_domain() };
    let train = train_input.as_domain();
    let seat_type = seat_type_input.as_domain();
    let reserve_type = reserve_type_input.as_domain();
    let departure_date = DepartureDate { value: NaiveDate::from_ymd_opt(y, m, d).unwrap() };
    let number_of_passengers = NumberOfPassengers { adult, child };

    // 片道おとな料金を算出する
    let single_trip_adult_fare = calc_single_trip_adult_fare(&ride_section, &train, &seat_type, &departure_date);

    // 適用される全体割引を判定する
    let overall_discount = judge_overall_discount(&number_of_passengers);

    // 適用される個別割引を判定する
    let individual_discounts =
        judge_individual_discounts(&ride_section, &number_of_passengers, departure_date.get_season());

    // 片道おとな料金に割引を適用して人数分の総料金を算出する
    calc_total_fare(
        &reserve_type,
        &number_of_passengers,
        &single_trip_adult_fare,
        &overall_discount,
        &individual_discounts,
    )
    .value
}

// presentation で domain に関与しないまま安全にバリデーション結果を受け渡すための enum

pub enum StationInput {
    Tokyo,
    #[allow(dead_code)]
    ShinOsaka,
    Himeji,
}

impl StationInput {
    fn as_domain(&self) -> Station {
        match self {
            Self::Tokyo => Station::Tokyo,
            Self::ShinOsaka => Station::ShinOsaka,
            Self::Himeji => Station::Himeji,
        }
    }
}

pub enum TrainInput {
    #[allow(dead_code)]
    Hikari,
    Nozomi,
}

impl TrainInput {
    fn as_domain(&self) -> Train {
        match self {
            Self::Hikari => Train::Hikari,
            Self::Nozomi => Train::Nozomi,
        }
    }
}

pub enum SeatTypeInput {
    Reserved,
    #[allow(dead_code)]
    Free,
}

impl SeatTypeInput {
    fn as_domain(&self) -> SeatType {
        match self {
            Self::Reserved => SeatType::Reserved,
            Self::Free => SeatType::Free,
        }
    }
}

pub enum ReserveTypeInput {
    #[allow(dead_code)]
    SingleTrip,
    RoundTrip,
}

impl ReserveTypeInput {
    fn as_domain(&self) -> ReserveType {
        match self {
            Self::SingleTrip => ReserveType::SingleTrip,
            Self::RoundTrip => ReserveType::RoundTrip,
        }
    }
}
