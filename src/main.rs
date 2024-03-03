use crate::application::{ReserveTypeInput, SeatTypeInput, StationInput, TrainInput};
use itertools::Itertools;

mod application;
mod domain;
mod fundamental;

fn main() {
    presentation();
}

fn presentation() {
    let amount = application::invoke(
        validate_departure(),
        validate_arrival(),
        validate_train(),
        validate_seat_type(),
        validate_reserve_type(),
        validate_y(),
        validate_m(),
        validate_d(),
        validate_adult(),
        validate_child(),
    );

    let result = format!("{amount:0}")
        .chars()
        .collect_vec()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect_vec()
        .join(",");

    println!("総料金は {result:0} 円です");
    // 運賃　　　　　: 10,010 * 往復割引 ( 0.9 ) => 9,000
    // 特急　　　　　: 6,450 + 繁忙期 ( 200 ) => 6,650
    // 片道おとな料金: 15,650
    // 片道こども料金: 7,820
    // おとな人数　　: 40 - 団体割引 ( 1 )
    // こども人数　　: 20
    // 往復　　　　　: x2
    // 総料金　　　　: (15,650 x 39) x 2 + (7,820 x 20) x 2 => 1,533,500
}

fn validate_departure() -> StationInput {
    StationInput::Tokyo
}

fn validate_arrival() -> StationInput {
    StationInput::Himeji
}

fn validate_train() -> TrainInput {
    TrainInput::Nozomi
}

fn validate_seat_type() -> SeatTypeInput {
    SeatTypeInput::Reserved
}

fn validate_reserve_type() -> ReserveTypeInput {
    ReserveTypeInput::RoundTrip
}

fn validate_y() -> i32 {
    2024
}

fn validate_m() -> u32 {
    12
}

fn validate_d() -> u32 {
    28
}

fn validate_adult() -> usize {
    40
}

fn validate_child() -> usize {
    20
}
