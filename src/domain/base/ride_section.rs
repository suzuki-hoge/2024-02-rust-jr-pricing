use crate::fundamental::operating_kilometer::OperatingKilometer;

pub struct RideSection {
    pub departure: Station,
    pub arrival: Station,
}

impl RideSection {
    pub fn get_station_pair(&self) -> (&Station, &Station) {
        let mut stations = vec![&self.departure, &self.arrival];
        stations.sort();

        (stations[0], stations[1])
    }

    pub fn get_operation_kilometer(&self) -> &OperatingKilometer {
        match self.get_station_pair() {
            (Station::Tokyo, Station::ShinOsaka) => &OperatingKilometer { value: 553.0 },
            (Station::Tokyo, Station::Himeji) => &OperatingKilometer { value: 644.0 },
            _ => panic!("unexpected ride section"),
        }
        // 線形的なデータ構造にした方がいいけど割愛
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Station {
    Tokyo,
    ShinOsaka,
    Himeji,
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::domain::base::ride_section::Station::{Himeji, ShinOsaka, Tokyo};
    use crate::domain::base::ride_section::{RideSection, Station};
    use crate::fundamental::operating_kilometer::OperatingKilometer;

    #[rstest]
    #[case(Tokyo, ShinOsaka, 553.0)]
    #[case(ShinOsaka, Tokyo, 553.0)]
    #[case(Tokyo, Himeji, 644.0)]
    #[case(Himeji, Tokyo, 644.0)]
    fn get_operation_kilometer(#[case] departure: Station, #[case] arrival: Station, #[case] exp: f64) {
        let sut = RideSection { departure, arrival };
        assert_eq!(&OperatingKilometer { value: exp }, sut.get_operation_kilometer());
    }
}
