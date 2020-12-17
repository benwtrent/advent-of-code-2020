use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum CubeState {
    On,
    Off,
}

impl From<&str> for CubeState {
    fn from(s: &str) -> Self {
        match s {
            "#" => CubeState::On,
            "." => CubeState::Off,
            _ => unimplemented!(),
        }
    }
}

impl Display for CubeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            CubeState::On => "#",
            CubeState::Off => ".",
        };
        write!(f, "{}", s)
    }
}

struct CoordinateDirection3d<'a> {
    coordinates: [i32; 3],
    direction: [i8; 3],
    maximum: &'a [i32; 3],
    minimum: &'a [i32; 3],
}

impl Iterator for CoordinateDirection3d<'_> {
    type Item = [i32; 3];

    fn next(&mut self) -> Option<[i32; 3]> {
        for (i, &d) in self.direction.iter().enumerate() {
            if (d < 0 && self.minimum[i] == self.coordinates[i])
                || (d > 0 && self.maximum[i] == self.coordinates[i])
            {
                return None;
            }
        }
        self.coordinates = [
            self.coordinates[0] + self.direction[0] as i32,
            self.coordinates[1] + self.direction[1] as i32,
            self.coordinates[2] + self.direction[2] as i32,
        ];
        Some(self.coordinates)
    }
}

struct CoordinateDirection4d<'a> {
    coordinates: [i32; 4],
    direction: [i8; 4],
    maximum: &'a [i32; 4],
    minimum: &'a [i32; 4],
}

impl Iterator for CoordinateDirection4d<'_> {
    type Item = [i32; 4];

    fn next(&mut self) -> Option<[i32; 4]> {
        for (i, &d) in self.direction.iter().enumerate() {
            if (d < 0 && self.minimum[i] == self.coordinates[i])
                || (d > 0 && self.maximum[i] == self.coordinates[i])
            {
                return None;
            }
        }
        self.coordinates = [
            self.coordinates[0] + self.direction[0] as i32,
            self.coordinates[1] + self.direction[1] as i32,
            self.coordinates[2] + self.direction[2] as i32,
            self.coordinates[3] + self.direction[3] as i32,
        ];
        Some(self.coordinates)
    }
}

#[derive(Debug, Copy, Clone)]
struct Cube {
    state: CubeState,
    coordinates: [i32; 3],
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{} {} {}]",
            self.state, self.coordinates[0], self.coordinates[1], self.coordinates[2]
        )
    }
}

#[derive(Debug, Copy, Clone)]
struct HyperCube {
    state: CubeState,
    coordinates: [i32; 4],
}

impl Display for HyperCube {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{} {} {} {}]",
            self.state,
            self.coordinates[0],
            self.coordinates[1],
            self.coordinates[2],
            self.coordinates[3]
        )
    }
}

fn all_coordinate_generators<'a>(
    cube: &'a Cube,
    maximum: &'a [i32; 3],
    minimum: &'a [i32; 3],
) -> Vec<CoordinateDirection3d<'a>> {
    let iterator_generator = |delta| CoordinateDirection3d {
        coordinates: cube.coordinates.clone(),
        maximum: &maximum,
        minimum: &minimum,
        direction: delta,
    };
    let mut v = vec![];
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if x != 0 || y != 0 || z != 0 {
                    v.push(iterator_generator([x, y, z]));
                }
            }
        }
    }
    v
}

fn all_4d_coordinate_generators<'a>(
    cube: &'a HyperCube,
    maximum: &'a [i32; 4],
    minimum: &'a [i32; 4],
) -> Vec<CoordinateDirection4d<'a>> {
    let iterator_generator = |delta| CoordinateDirection4d {
        coordinates: cube.coordinates.clone(),
        maximum: &maximum,
        minimum: &minimum,
        direction: delta,
    };
    let mut v = vec![];
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    if x != 0 || y != 0 || z != 0 || w != 0 {
                        v.push(iterator_generator([x, y, z, w]));
                    }
                }
            }
        }
    }
    v
}

const DEFAULT_CUBE: Cube = Cube {
    coordinates: [0; 3],
    state: CubeState::Off,
};

const DEFAULT_HYPER_CUBE: HyperCube = HyperCube {
    coordinates: [0; 4],
    state: CubeState::Off,
};

impl Cube {
    fn new_state(
        &self,
        max_coordinates: &[i32; 3],
        min_coordinates: &[i32; 3],
        space: &HashMap<[i32; 3], Cube>,
    ) -> Self {
        let mut visual_iters = all_coordinate_generators(&self, max_coordinates, min_coordinates);
        let mut active_count = 0;
        for coor_iter in visual_iters.iter_mut() {
            if let Some(c) = coor_iter.next() {
                if space.get(&c).unwrap_or(&DEFAULT_CUBE).state == CubeState::On {
                    active_count += 1;
                }
            }
        }
        let mut state = self.state.clone();
        if self.state == CubeState::On {
            if !(active_count == 3 || active_count == 2) {
                state = CubeState::Off;
            }
        }
        if self.state == CubeState::Off && active_count == 3 {
            state = CubeState::On;
        }
        Cube {
            coordinates: self.coordinates.clone(),
            state,
        }
    }
}

impl HyperCube {
    fn new_state(
        &self,
        max_coordinates: &[i32; 4],
        min_coordinates: &[i32; 4],
        space: &HashMap<[i32; 4], HyperCube>,
    ) -> Self {
        let mut visual_iters =
            all_4d_coordinate_generators(&self, max_coordinates, min_coordinates);
        let mut active_count = 0;
        for coor_iter in visual_iters.iter_mut() {
            if let Some(c) = coor_iter.next() {
                if space.get(&c).unwrap_or(&DEFAULT_HYPER_CUBE).state == CubeState::On {
                    active_count += 1;
                }
            }
        }
        let mut state = self.state.clone();
        if self.state == CubeState::On {
            if !(active_count == 3 || active_count == 2) {
                state = CubeState::Off;
            }
        }
        if self.state == CubeState::Off && active_count == 3 {
            state = CubeState::On;
        }
        HyperCube {
            coordinates: self.coordinates.clone(),
            state,
        }
    }
}

#[aoc_generator(day17, part1)]
fn to_vec(input: &str) -> (HashMap<[i32; 3], Cube>, [i32; 3]) {
    let mut space: HashMap<[i32; 3], Cube> = HashMap::new();
    let mut y = 0;
    let mut max_x = 0;
    for l in input.lines() {
        for (x, s) in l.split("").filter(|&s| !s.is_empty()).enumerate() {
            space.insert(
                [x as i32, y, 0],
                Cube {
                    state: s.into(),
                    coordinates: [x as i32, y, 0],
                },
            );
            max_x = x.max(max_x);
        }
        y += 1;
    }
    (space, [max_x as i32, y, 0])
}

#[aoc(day17, part1)]
fn active_state_count(input: &(HashMap<[i32; 3], Cube>, [i32; 3])) -> usize {
    let mut curr_vals = input.0.clone();
    let mut z_max = 1;
    let mut z_min = -1;
    let mut y_max = input.1[1] + 1;
    let mut y_min = -1;
    let mut x_max = input.1[0] + 1;
    let mut x_min = -1;
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            for z in z_min..=z_max {
                let coors = [x, y, z];
                curr_vals.entry(coors.clone()).or_insert(Cube {
                    coordinates: coors,
                    state: CubeState::Off,
                });
            }
        }
    }
    for &x in &[-1, input.1[0]] {
        for &y in &[-1, input.1[1]] {
            let coors = [x, y, 0];
            curr_vals.insert(
                coors.clone(),
                Cube {
                    coordinates: coors,
                    state: CubeState::Off,
                },
            );
        }
    }
    for _ in 0..6 {
        let mut new_vals = curr_vals.clone();
        let max_coors = [x_max, y_max, z_max];
        let min_coors = [x_min, y_min, z_min];
        for (&coors, &cube) in &curr_vals {
            let new_cube = cube.new_state(&max_coors, &min_coors, &curr_vals);
            new_vals.insert(coors, new_cube);
            if new_cube.state == CubeState::On {
                z_max = z_max.max(new_cube.coordinates[2] + 1);
                y_max = y_max.max(new_cube.coordinates[1] + 1);
                x_max = x_max.max(new_cube.coordinates[0] + 1);
                z_min = z_min.min(new_cube.coordinates[2] - 1);
                y_min = y_min.min(new_cube.coordinates[1] - 1);
                x_min = x_min.min(new_cube.coordinates[0] - 1);
            }
        }
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    let coors = [x, y, z];
                    new_vals.entry(coors.clone()).or_insert(Cube {
                        coordinates: coors,
                        state: CubeState::Off,
                    });
                }
            }
        }
        curr_vals = new_vals;
    }
    curr_vals
        .values()
        .filter(|&c| c.state == CubeState::On)
        .count()
}

#[aoc_generator(day17, part2)]
fn to_hyper_vec(input: &str) -> (HashMap<[i32; 4], HyperCube>, [i32; 4]) {
    let mut space: HashMap<[i32; 4], HyperCube> = HashMap::new();
    let mut y = 0;
    let mut max_x = 0;
    for l in input.lines() {
        for (x, s) in l.split("").filter(|&s| !s.is_empty()).enumerate() {
            space.insert(
                [x as i32, y, 0, 0],
                HyperCube {
                    state: s.into(),
                    coordinates: [x as i32, y, 0, 0],
                },
            );
            max_x = x.max(max_x);
        }
        y += 1;
    }
    (space, [max_x as i32, y, 0, 0])
}

#[aoc(day17, part2)]
fn hyper_active_state_count(input: &(HashMap<[i32; 4], HyperCube>, [i32; 4])) -> usize {
    let mut curr_vals = input.0.clone();
    let mut w_max = 1;
    let mut w_min = -1;
    let mut z_max = 1;
    let mut z_min = -1;
    let mut y_max = input.1[1] + 1;
    let mut y_min = -1;
    let mut x_max = input.1[0] + 1;
    let mut x_min = -1;
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            for z in z_min..=z_max {
                for w in z_min..=z_max {
                    let coors = [x, y, z, w];
                    curr_vals.entry(coors.clone()).or_insert(HyperCube {
                        coordinates: coors,
                        state: CubeState::Off,
                    });
                }
            }
        }
    }
    for &x in &[-1, input.1[0]] {
        for &y in &[-1, input.1[1]] {
            let coors = [x, y, 0, 0];
            curr_vals.insert(
                coors.clone(),
                HyperCube {
                    coordinates: coors,
                    state: CubeState::Off,
                },
            );
        }
    }
    for _ in 0..6 {
        let mut new_vals = curr_vals.clone();
        let max_coors = [x_max, y_max, z_max, w_max];
        let min_coors = [x_min, y_min, z_min, w_min];
        for (&coors, &cube) in &curr_vals {
            let new_cube = cube.new_state(&max_coors, &min_coors, &curr_vals);
            new_vals.insert(coors, new_cube);
            if new_cube.state == CubeState::On {
                w_max = w_max.max(new_cube.coordinates[3] + 1);
                z_max = z_max.max(new_cube.coordinates[2] + 1);
                y_max = y_max.max(new_cube.coordinates[1] + 1);
                x_max = x_max.max(new_cube.coordinates[0] + 1);
                w_min = w_min.min(new_cube.coordinates[3] - 1);
                z_min = z_min.min(new_cube.coordinates[2] - 1);
                y_min = y_min.min(new_cube.coordinates[1] - 1);
                x_min = x_min.min(new_cube.coordinates[0] - 1);
            }
        }
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    for w in w_min..=w_max {
                        let coors = [x, y, z, w];
                        new_vals.entry(coors.clone()).or_insert(HyperCube {
                            coordinates: coors,
                            state: CubeState::Off,
                        });
                    }
                }
            }
        }
        curr_vals = new_vals;
    }
    curr_vals
        .values()
        .filter(|&c| c.state == CubeState::On)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = ".#.
..#
###
";

    #[test]
    fn test_count_after_cycles() {
        assert_eq!(active_state_count(&to_vec(TEST_INPUT)), 112);
        assert_eq!(hyper_active_state_count(&to_hyper_vec(TEST_INPUT)), 848);
    }
}
