#[derive(Debug)]
enum Movement {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl From<&str> for Movement {
    fn from(s: &str) -> Self {
        let v = String::from(&s[..1]);
        let val: i32 = (&s[1..]).parse().unwrap();
        match v.as_str() {
            "N" => Movement::North(val),
            "S" => Movement::South(val),
            "E" => Movement::East(val),
            "W" => Movement::West(val),
            "L" => Movement::Left(val),
            "R" => Movement::Right(val),
            "F" => Movement::Forward(val),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
enum Facing {
    N,
    S,
    E,
    W,
}

impl Facing {
    fn new_direction(&self, degrees: &i32) -> Facing {
        match self {
            Facing::N => match degrees {
                90 => Facing::E,
                180 => Facing::S,
                270 => Facing::W,
                _ => unimplemented!(),
            },
            Facing::S => match degrees {
                90 => Facing::W,
                180 => Facing::N,
                270 => Facing::E,
                _ => unimplemented!(),
            },
            Facing::E => match degrees {
                90 => Facing::S,
                180 => Facing::W,
                270 => Facing::N,
                _ => unimplemented!(),
            },
            Facing::W => match degrees {
                90 => Facing::N,
                180 => Facing::E,
                270 => Facing::S,
                _ => unimplemented!(),
            },
        }
    }
}

#[derive(Debug)]
struct Position {
    facing: Facing,
    x: i32,
    y: i32,
    waypoint_x: i32,
    waypoint_y: i32,
}

impl Position {
    fn new() -> Self {
        Position {
            facing: Facing::E,
            x: 0,
            y: 0,
            waypoint_x: 10,
            waypoint_y: 1,
        }
    }

    fn travel(&mut self, movement: &Movement) {
        match movement {
            Movement::North(val) => self.y += *val,
            Movement::South(val) => self.y -= *val,
            Movement::East(val) => self.x += *val,
            Movement::West(val) => self.x -= *val,
            Movement::Left(val) => self.facing = self.facing.new_direction(&(360 - val)),
            Movement::Right(val) => self.facing = self.facing.new_direction(val),
            Movement::Forward(val) => match self.facing {
                Facing::N => self.y += *val,
                Facing::S => self.y -= *val,
                Facing::E => self.x += *val,
                Facing::W => self.x -= *val,
            },
        }
    }

    fn travel_waypoint(&mut self, movement: &Movement) {
        match movement {
            Movement::North(val) => self.waypoint_y += *val,
            Movement::South(val) => self.waypoint_y -= *val,
            Movement::East(val) => self.waypoint_x += *val,
            Movement::West(val) => self.waypoint_x -= *val,
            Movement::Left(val) => self.rotate_waypoint(&(360 - val)),
            Movement::Right(val) => self.rotate_waypoint(val),
            Movement::Forward(val) => {
                self.x += val * self.waypoint_x;
                self.y += val * self.waypoint_y;
            }
        }
    }

    fn rotate_waypoint(&mut self, degrees: &i32) {
        match degrees {
            90 => {
                let new_x = self.waypoint_y;
                let new_y = -self.waypoint_x;
                self.waypoint_y = new_y;
                self.waypoint_x = new_x;
            }
            180 => {
                self.waypoint_x = -self.waypoint_x;
                self.waypoint_y = -self.waypoint_y;
            }
            270 => {
                let new_x = -self.waypoint_y;
                let new_y = self.waypoint_x;
                self.waypoint_y = new_y;
                self.waypoint_x = new_x;
            }
            _ => unimplemented!(),
        };
    }
}

#[aoc_generator(day12)]
fn to_vec(input: &str) -> Vec<Movement> {
    input.lines().map(|s| s.into()).collect()
}

#[aoc(day12, part1)]
fn manhatten_movement(input: &Vec<Movement>) -> usize {
    let mut position = Position::new();
    for movement in input {
        position.travel(movement);
    }
    return (position.x.abs() + position.y.abs()) as usize;
}

#[aoc(day12, part2)]
fn manhatten_waypoint_movement(input: &Vec<Movement>) -> usize {
    let mut position = Position::new();
    for movement in input {
        position.travel_waypoint(movement);
    }
    return (position.x.abs() + position.y.abs()) as usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_travel() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(manhatten_movement(&input), 25);
    }

    #[test]
    fn test_waypoint_travel() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(manhatten_waypoint_movement(&input), 286);
    }
}
