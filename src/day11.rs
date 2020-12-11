#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
    Occupied,
    Unoccupied,
    Floor,
}

impl From<&str> for State {
    fn from(s: &str) -> Self {
        match s {
            "L" => State::Unoccupied,
            "#" => State::Occupied,
            "." => State::Floor,
            _ => unimplemented!(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Seat {
    coordinates: (usize, usize),
    state: State,
}

struct CoordinateDirection<'a> {
    coordinates: (usize, usize),
    direction: (i8, i8),
    maximum: &'a (usize, usize),
}

impl Iterator for CoordinateDirection<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if (self.direction.0 < 0 && self.coordinates.0 == 0)
            || (self.direction.0 > 0 && self.coordinates.0 >= self.maximum.0)
            || (self.direction.1 < 0 && self.coordinates.1 == 0)
            || (self.direction.1 > 0 && self.coordinates.1 >= self.maximum.1)
        {
            return None;
        }

        self.coordinates = (
            (self.coordinates.0 as i8 + self.direction.0) as usize,
            (self.coordinates.1 as i8 + self.direction.1) as usize,
        );
        Some(self.coordinates)
    }
}

fn all_coordinate_generators<'a>(
    seat: &'a Seat,
    maximum: &'a (usize, usize),
) -> Vec<CoordinateDirection<'a>> {
    let iterator_generator = |(dx, dy)| CoordinateDirection {
        coordinates: seat.coordinates.clone(),
        maximum: &maximum,
        direction: (dx, dy),
    };
    vec![
        iterator_generator((1, 0)),
        iterator_generator((1, 1)),
        iterator_generator((1, -1)),
        iterator_generator((-1, 0)),
        iterator_generator((-1, 1)),
        iterator_generator((-1, -1)),
        iterator_generator((0, 1)),
        iterator_generator((0, -1)),
    ]
}

fn new_state(seat: &Seat, arrangement: &Vec<Vec<Seat>>) -> Seat {
    if seat.state == State::Floor {
        return seat.clone();
    }
    let maximum = (arrangement[0].len() - 1, arrangement.len() - 1);
    let mut visual_iters = all_coordinate_generators(seat, &maximum);
    let mut occupied_count = 0;
    for coor_iter in visual_iters.iter_mut() {
        if let Some((x, y)) = coor_iter.next() {
            if arrangement[y][x].state == State::Occupied {
                occupied_count += 1;
            }
        }
    }
    let state = if occupied_count >= 4 && seat.state == State::Occupied {
        State::Unoccupied
    } else if occupied_count == 0 && seat.state == State::Unoccupied {
        State::Occupied
    } else {
        seat.state.clone()
    };
    Seat {
        coordinates: seat.coordinates.clone(),
        state,
    }
}

fn new_state_visually(seat: &Seat, arrangement: &Vec<Vec<Seat>>) -> Seat {
    if seat.state == State::Floor {
        return seat.clone();
    }
    let maximum = (arrangement[0].len() - 1, arrangement.len() - 1);
    let mut visual_iters = all_coordinate_generators(seat, &maximum);
    let mut occupied_count = 0;
    for coor_iter in visual_iters.iter_mut() {
        if let Some((x, y)) = coor_iter
            .skip_while(|(x, y)| arrangement[*y][*x].state == State::Floor)
            .next()
        {
            if arrangement[y][x].state == State::Occupied {
                occupied_count += 1;
            }
        }
    }
    let state = if occupied_count > 4 && seat.state == State::Occupied {
        State::Unoccupied
    } else if occupied_count == 0 && seat.state == State::Unoccupied {
        State::Occupied
    } else {
        seat.state.clone()
    };
    Seat {
        coordinates: seat.coordinates.clone(),
        state,
    }
}

#[aoc_generator(day11)]
fn to_vec(input: &str) -> Vec<Vec<Seat>> {
    let mut seats = vec![];
    for (ys, vals) in input.lines().enumerate() {
        let mut row = vec![];
        for (xs, state) in vals.split("").filter(|s| !(*s).is_empty()).enumerate() {
            let seat = Seat {
                state: state.into(),
                coordinates: (xs, ys),
            };
            row.push(seat);
        }
        seats.push(row);
    }
    seats
}

fn reach_stability_count(
    input: &Vec<Vec<Seat>>,
    state_check: &dyn Fn(&Seat, &Vec<Vec<Seat>>) -> Seat,
) -> usize {
    let mut old_arrangement = input.clone();
    loop {
        let mut new_arrangement = vec![];
        for row in old_arrangement.iter() {
            let mut new_row = vec![];
            for seat in row.iter() {
                new_row.push(state_check(&seat, &old_arrangement));
            }
            new_arrangement.push(new_row);
        }
        if new_arrangement == old_arrangement {
            break;
        }
        old_arrangement = new_arrangement;
    }
    old_arrangement
        .iter()
        .flat_map(|v| v)
        .filter(|s| (*s).state == State::Occupied)
        .count()
}

#[aoc(day11, part1)]
fn occupied_seats_in_stability(input: &Vec<Vec<Seat>>) -> usize {
    reach_stability_count(input, &new_state)
}

#[aoc(day11, part2)]
fn occupied_seats_in_visual_stability(input: &Vec<Vec<Seat>>) -> usize {
    reach_stability_count(input, &new_state_visually)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

    #[test]
    fn test_stability_count() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(occupied_seats_in_stability(&input), 37);
    }

    #[test]
    fn test_visual_stability_count() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(occupied_seats_in_visual_stability(&input), 26);
    }
}
