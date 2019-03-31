#[derive(Clone, Copy, Debug, PartialEq)]
struct Location {
    x: usize,
    y: usize,
}

fn get_path_to(
    current_location: Location,
    target_location: Location,
    off_limits: &[Location],
) -> Vec<Location> {
    if current_location == target_location {
        return vec![target_location];
    }
    if current_location.y < target_location.y {
        let location_down = Location {
            x: current_location.x,
            y: current_location.y + 1,
        };
        if !off_limits.contains(&location_down) {
            // location_down
            let mut path = get_path_to(location_down, target_location, off_limits);
            if let Some(last_location) = path.last() {
                if last_location == &target_location {
                    path.insert(0, location_down);
                    return path;
                }
            }
        }
    }
    if current_location.x < target_location.x {
        let location_right = Location {
            x: current_location.x + 1,
            y: current_location.y,
        };
        if !off_limits.contains(&location_right) {
            // location_right
            let mut path = get_path_to(location_right, target_location, off_limits);
            if let Some(last_location) = path.last() {
                if last_location == &target_location {
                    path.insert(0, location_right);
                    return path;
                }
            }
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_robot_in_a_grid() {
        let mut off_limits: Vec<Location> = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let x = rng.gen_range(1, 100);
            let y = rng.gen_range(1, 100);
            off_limits.push(Location { x, y });
        }
        let path = get_path_to(
            Location { x: 0, y: 0 },
            Location { x: 100, y: 100 },
            &off_limits,
        );
        assert_eq!(path.last().unwrap(), &Location { x: 100, y: 100 });
    }
}

fn main() {
    get_path_to(Location { x: 0, y: 0 }, Location { x: 100, y: 100 }, &[]);
}
