use crate::domain::base::departure_date::DepartureDate;

use crate::domain::base::ride_section::RideSection;
use crate::domain::base::seat_type::SeatType;
use crate::domain::base::train::Train;

use crate::domain::fare::express_fare::{calc_express_fare, ExpressFare};
use crate::domain::fare::train_fare::{calc_train_fare, TrainFare};

pub struct SingleTripAdultFare {
    pub train_fare: TrainFare,
    pub express_fare: ExpressFare,
}

impl SingleTripAdultFare {
    pub fn get_adult_fare(&self) -> (TrainFare, ExpressFare) {
        (self.train_fare.clone(), self.express_fare.clone())
    }

    pub fn get_child_fare(&self) -> (TrainFare, ExpressFare) {
        (
            TrainFare { value: self.train_fare.clone().value * 0.5 },
            ExpressFare { value: self.express_fare.clone().value * 0.5 },
        )
    }
}

pub fn calc_single_trip_adult_fare(
    ride_section: &RideSection,
    train: &Train,
    seat_type: &SeatType,
    departure_date: &DepartureDate,
) -> SingleTripAdultFare {
    let train_fare = calc_train_fare(ride_section);
    let express_fare = calc_express_fare(train, seat_type, ride_section, departure_date);
    SingleTripAdultFare { train_fare, express_fare }
}
