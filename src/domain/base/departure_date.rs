use chrono::{Datelike, NaiveDate};

use crate::domain::base::departure_date::Season::{OffPeak, Peak, Regular};

pub struct DepartureDate {
    pub value: NaiveDate,
}

impl DepartureDate {
    pub fn get_season(&self) -> &Season {
        let md = (self.value.month(), self.value.day());
        if ((1, 16)..=(1, 30)).contains(&md) {
            &OffPeak
        } else if (12, 25) <= md || md <= (1, 10) {
            &Peak
        } else {
            &Regular
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Season {
    Regular,
    OffPeak,
    Peak,
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use rstest::rstest;

    use crate::domain::base::departure_date::Season::{OffPeak, Peak, Regular};
    use crate::domain::base::departure_date::{DepartureDate, Season};

    #[rstest]
    #[case(01, 01, Peak)]
    #[case(01, 10, Peak)]
    #[case(01, 11, Regular)]
    #[case(01, 15, Regular)]
    #[case(01, 16, OffPeak)]
    #[case(01, 30, OffPeak)]
    #[case(02, 01, Regular)]
    #[case(12, 24, Regular)]
    #[case(12, 25, Peak)]
    #[case(12, 31, Peak)]
    fn get_season(#[case] m: u32, #[case] d: u32, #[case] exp: Season) {
        let sut = DepartureDate { value: NaiveDate::from_ymd_opt(2024, m, d).unwrap() };
        assert_eq!(&exp, sut.get_season());
    }
}
