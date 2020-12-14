use std::collections::HashMap;
use std::usize::MAX;

#[aoc_generator(day13)]
fn to_vec(input: &str) -> (usize, HashMap<usize, usize>) {
    let arrival: usize = input.lines().next().unwrap().parse().unwrap();
    let mut busses = HashMap::new();
    for (i, s) in input.lines().skip(1).next().unwrap().split(",").enumerate() {
        if s != "x" {
            busses.insert(i, s.parse().unwrap());
        }
    }
    (arrival, busses)
}

#[aoc(day13, part1)]
fn bus_wait_time(input: &(usize, HashMap<usize, usize>)) -> usize {
    let time = input.0;
    let busses = &input.1;
    let mut min_time = MAX;
    let mut best_bus_id = 0;
    for (&i, bus) in busses.iter() {
        let r = time % *bus;
        let waiting = *bus + time - r;
        if waiting < min_time {
            min_time = waiting;
            best_bus_id = i;
        }
    }
    busses[&best_bus_id] * (min_time - time)
}

#[aoc(day13, part2)]
fn magic_timestamp(input: &(usize, HashMap<usize, usize>)) -> usize {
    let mut buses: Vec<(usize, usize)> = input.1.iter().map(|(&pos, &id)| (pos, id)).collect();
    buses.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut timestamp = 0;
    let mut inc = buses[0].1;
    for &(i, bus) in &buses[1..] {
        // friggin CRT sieve garbage see: https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Computation
        while (timestamp + i) % bus != 0 {
            timestamp += inc;
        }
        // adjust for the next modulo
        inc *= bus;
    }
    timestamp
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn earliest_bus() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(bus_wait_time(&input), 295);
    }

    #[test]
    fn test_magic_timestamp() {
        let input = to_vec(TEST_INPUT);
        let result = magic_timestamp(&input);
        assert_eq!(result, 1068781);
    }
}
