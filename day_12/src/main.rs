use shared;

#[derive(Debug)]
struct NavigatorV1<'a> {
    facing_direction: &'a str,
    north: usize,
    west: usize,
    east: usize,
    south: usize,
}

impl NavigatorV1<'_> {
    fn new(facing_direction: &str) -> NavigatorV1 {
        NavigatorV1 {
            facing_direction,
            north: 0,
            west: 0,
            east: 0,
            south: 0,
        }
    }
    fn rotate(&mut self, direction: &str, deg: usize) {
        let num_shifts = deg / 90;
        let all_directions = ["N", "E", "S", "W"];

        let mut current_idx = all_directions
            .iter()
            .position(|&d| d == self.facing_direction)
            .unwrap();

        for _i in 0..num_shifts {
            match direction {
                "R" => {
                    current_idx = (current_idx + 1) % all_directions.len();
                }
                "L" => {
                    if current_idx as isize - 1 >= 0 {
                        current_idx -= 1;
                    } else {
                        current_idx = all_directions.len() - 1;
                    }
                }
                _ => panic!("Unknown direction {}", direction),
            }
        }

        self.facing_direction = all_directions[current_idx];
    }
    pub fn nav(&mut self, action: &str) {
        let cmd = &action[0..1];
        let distance = action[1..].parse::<usize>().unwrap();

        match cmd {
            "N" => {
                let new_south = self.south as isize - distance as isize;

                if new_south > 0 {
                    self.south = new_south as usize;
                } else {
                    self.north += distance - self.south;
                    self.south = 0;
                }
            }
            "S" => {
                let new_north = self.north as isize - distance as isize;

                if new_north > 0 {
                    self.north = new_north as usize;
                } else {
                    self.south += distance - self.north;
                    self.north = 0;
                }
            }
            "E" => {
                let new_west = self.west as isize - distance as isize;

                if new_west > 0 {
                    self.west = new_west as usize;
                } else {
                    self.east += distance - self.west;
                    self.west = 0;
                }
            }
            "W" => {
                let new_east = self.east as isize - distance as isize;

                if new_east > 0 {
                    self.east = new_east as usize;
                } else {
                    self.west += distance - self.east;
                    self.east = 0;
                }
            }
            "L" | "R" => self.rotate(cmd, distance),
            "F" => {
                let new_action = format!("{}{}", self.facing_direction, distance);
                self.nav(&new_action);
            }
            _ => panic!("Unknown command {}", cmd),
        }
    }

    pub fn manhattan_distance(&self) -> usize {
        self.north + self.west + self.east + self.south
    }
}

#[derive(Debug)]
struct Point {
    pub north: usize,
    pub west: usize,
    pub east: usize,
    pub south: usize,
}

impl Point {
    pub fn set_direction(&mut self, direction: &str, value: usize) {
        match direction {
            "N" => self.north = value,
            "W" => self.west = value,
            "E" => self.east = value,
            "S" => self.south = value,
            _ => panic!("Unknown direction {}", direction),
        }
    }

    pub fn get_direction(&self, direction: &str) -> usize {
        match direction {
            "N" => self.north,
            "W" => self.west,
            "E" => self.east,
            "S" => self.south,
            _ => panic!("Unknown direction {}", direction),
        }
    }
}
#[derive(Debug)]
struct NavigatorV2<'a> {
    facing_direction: &'a str,
    ship: Point,
    waypoint: Point,
}

impl NavigatorV2<'_> {
    fn new(facing_direction: &str) -> NavigatorV2 {
        let ship = Point {
            north: 0,
            west: 0,
            east: 0,
            south: 0,
        };

        let waypoint = Point {
            north: 1,
            west: 0,
            east: 10,
            south: 0,
        };

        NavigatorV2 {
            facing_direction,
            ship,
            waypoint,
        }
    }
    fn rotate(&mut self, direction: &str, deg: usize) {
        let num_shifts = deg / 90;
        let all_directions = ["N", "E", "S", "W"];

        let mut new_waypoint = Point {
            north: 0,
            west: 0,
            east: 0,
            south: 0,
        };

        for curr_dir in all_directions.iter() {
            let mut current_idx = all_directions.iter().position(|&d| d == *curr_dir).unwrap();

            for _i in 0..num_shifts {
                match direction {
                    "R" => {
                        current_idx = (current_idx + 1) % all_directions.len();
                    }
                    "L" => {
                        if current_idx as isize - 1 >= 0 {
                            current_idx -= 1;
                        } else {
                            current_idx = all_directions.len() - 1;
                        }
                    }
                    _ => panic!("Unknown rotation {}", direction),
                }
            }

            let curr_value = self.waypoint.get_direction(curr_dir);

            let new_direction = all_directions[current_idx];
            new_waypoint.set_direction(new_direction, curr_value);
        }

        self.waypoint = new_waypoint;
    }
    pub fn nav(&mut self, action: &str) {
        let cmd = &action[0..1];
        let amount = action[1..].parse::<usize>().unwrap();

        match cmd {
            "N" => {
                let new_south = self.waypoint.south as isize - amount as isize;

                if new_south > 0 {
                    self.waypoint.south = new_south as usize;
                } else {
                    self.waypoint.north += amount - self.waypoint.south;
                    self.waypoint.south = 0;
                }
            }
            "S" => {
                let new_north = self.waypoint.north as isize - amount as isize;

                if new_north > 0 {
                    self.waypoint.north = new_north as usize;
                } else {
                    self.waypoint.south += amount - self.waypoint.north;
                    self.waypoint.north = 0;
                }
            }
            "E" => {
                let new_west = self.waypoint.west as isize - amount as isize;

                if new_west > 0 {
                    self.waypoint.west = new_west as usize;
                } else {
                    self.waypoint.east += amount - self.waypoint.west;
                    self.waypoint.west = 0;
                }
            }
            "W" => {
                let new_east = self.waypoint.east as isize - amount as isize;

                if new_east > 0 {
                    self.waypoint.east = new_east as usize;
                } else {
                    self.waypoint.west += amount - self.waypoint.east;
                    self.waypoint.east = 0;
                }
            }
            "L" | "R" => self.rotate(cmd, amount),
            "F" => {
                self.ship.north = self.ship.north + amount * self.waypoint.north;
                self.ship.south = self.ship.south + amount * self.waypoint.south;
                if self.ship.north >= self.ship.south {
                    self.ship.north = self.ship.north - self.ship.south;
                    self.ship.south = 0;
                } else {
                    self.ship.south = self.ship.south - self.ship.north;
                    self.ship.north = 0;
                }

                self.ship.east = self.ship.east + amount * self.waypoint.east;
                self.ship.west = self.ship.west + amount * self.waypoint.west;

                if self.ship.east >= self.ship.west {
                    self.ship.east = self.ship.east - self.ship.west;
                    self.ship.west = 0;
                } else {
                    self.ship.west = self.ship.west - self.ship.east;
                    self.ship.east = 0;
                }
            }
            _ => panic!("Unknown command {}", cmd),
        }
    }

    pub fn manhattan_distance(&self) -> usize {
        self.ship.north + self.ship.west + self.ship.east + self.ship.south
    }
}

fn main() {
    let actions = shared::read_file("input.txt");
    let mut navigator = NavigatorV1::new("E");
    for action in actions {
        navigator.nav(&action);
    }

    println!(
        "Manhattan distance between location and starting position using method 1 is {}",
        navigator.manhattan_distance()
    );

    let actions = shared::read_file("input.txt");
    let mut navigator2 = NavigatorV2::new("E");
    for action in actions {
        navigator2.nav(&action);
    }

    println!(
        "Manhattan distance between location and starting position using method 2 is {}",
        navigator2.manhattan_distance()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_manhattan_distance() {
        let actions = shared::parse_input_to_string_vec(
            "
        F10
        N3
        F7
        R90
        F11",
        );

        let mut navigator = NavigatorV1::new("E");

        for action in actions {
            navigator.nav(&action);
        }

        assert_eq!(navigator.manhattan_distance(), 25);
    }

    #[test]
    fn it_calculates_v2_manhattan_distance() {
        let actions = shared::parse_input_to_string_vec(
            "
        F10
        N3
        F7
        R90
        F11",
        );

        let mut navigator = NavigatorV2::new("E");

        for action in actions {
            navigator.nav(&action);
        }

        assert_eq!(navigator.manhattan_distance(), 286);
    }
}
