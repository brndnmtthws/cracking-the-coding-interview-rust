#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Occupied,
    Unoccupied,
}

#[derive(Debug, Clone)]
struct ParkingSpace {
    state: State,
    location: String,
}

#[derive(Debug, Clone)]
struct ParkingLot {
    parking_spaces: Vec<ParkingSpace>,
}

impl ParkingLot {
    fn new(section_count: u8, spaces_per_section: usize) -> ParkingLot {
        let mut spaces: Vec<ParkingSpace> = vec![];
        for i in 0..section_count {
            let section = (65 + i) as char;
            for j in 0..spaces_per_section {
                spaces.push(ParkingSpace {
                    state: State::Unoccupied,
                    location: format!("{}{}", section, j + 1),
                })
            }
        }
        ParkingLot {
            parking_spaces: spaces,
        }
    }

    fn occupy_space(&mut self, location: &str) {
        if let Some(space) = self
            .parking_spaces
            .iter_mut()
            .find(|space| space.location == location)
        {
            space.state = State::Occupied;
        }
    }

    fn release_space(&mut self, location: &str) {
        if let Some(space) = self
            .parking_spaces
            .iter_mut()
            .find(|space| space.location == location)
        {
            space.state = State::Unoccupied;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parking_lot() {
        let mut parking_lot = ParkingLot::new(3, 6);
        parking_lot.occupy_space("A1");
        assert_eq!(parking_lot.parking_spaces[0].state, State::Occupied);
        parking_lot.release_space("A1");
        assert_eq!(parking_lot.parking_spaces[0].state, State::Unoccupied);
    }
}

fn main() {
    let mut parking_lot = ParkingLot::new(3, 6);
    parking_lot.occupy_space("A1");
    assert_eq!(parking_lot.parking_spaces[0].state, State::Occupied);
    parking_lot.release_space("A1");
}
