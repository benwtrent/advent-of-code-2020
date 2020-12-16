use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq)]
struct Field {
    name: String,
    range_1: (usize, usize),
    range_2: (usize, usize),
}

impl From<&str> for Field {
    fn from(s: &str) -> Self {
        let mut colon = s.split(":");
        let name = colon.next().unwrap();
        let ranges = colon.next().unwrap();
        let mut range_strs = ranges.split(" or ");
        let range_1: Vec<usize> = range_strs
            .next()
            .unwrap()
            .split("-")
            .map(|split| split.trim().parse().unwrap())
            .collect();
        let range_2: Vec<usize> = range_strs
            .next()
            .unwrap()
            .split("-")
            .map(|split| split.trim().parse().unwrap())
            .collect();
        Field {
            name: String::from(name),
            range_1: (range_1[0], range_1[1]),
            range_2: (range_2[0], range_2[1]),
        }
    }
}

impl Field {
    fn is_invalid_number(&self, n: &usize) -> bool {
        !((*n >= self.range_1.0 && *n <= self.range_1.1)
            || (*n >= self.range_2.0 && *n <= self.range_2.1))
    }
}

#[aoc_generator(day16)]
fn to_vec(input: &str) -> (Vec<Field>, Vec<usize>, Vec<Vec<usize>>) {
    let mut splits = input.split("\n\n");
    let fields: Vec<Field> = splits.next().unwrap().lines().map(|s| s.into()).collect();
    let my_ticket: Vec<usize> = splits
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let other_tickets: Vec<Vec<usize>> = splits
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|s| s.split(",").map(|s| s.parse().unwrap()).collect())
        .collect();
    (fields, my_ticket, other_tickets)
}

#[aoc(day16, part1)]
fn sum_invalid_tickets(input: &(Vec<Field>, Vec<usize>, Vec<Vec<usize>>)) -> usize {
    let mut sum = 0;
    for vec in &input.2 {
        for &v in vec {
            if input.0.iter().all(|f| f.is_invalid_number(&v)) {
                sum += v;
            }
        }
    }
    sum
}

#[aoc(day16, part2)]
fn departure_products(input: &(Vec<Field>, Vec<usize>, Vec<Vec<usize>>)) -> usize {
    let valid_tickets: Vec<&Vec<usize>> = input
        .2
        .iter()
        .filter(|&vec| {
            !vec.iter()
                .any(|v| input.0.iter().all(|f| f.is_invalid_number(&v)))
        })
        .collect();
    let mut field_translation = HashMap::new();
    let mut translated_fields = HashSet::new();
    while field_translation.len() < input.0.len() {
        for f in input.0.iter() {
            if translated_fields.contains(f) {
                continue;
            }
            let mut matched = false;
            let mut matched_more = false;
            let mut column = 0;
            for j in 0..input.0.len() {
                if field_translation.contains_key(&j) {
                    continue;
                }
                if !valid_tickets.iter().any(|vec| f.is_invalid_number(&vec[j])) {
                    if matched {
                        matched_more = true;
                        break;
                    }
                    matched = true;
                    column = j;
                }
            }
            if matched && !matched_more {
                field_translation.insert(column, f);
                translated_fields.insert(f);
            }
        }
    }

    input
        .1
        .iter()
        .enumerate()
        .filter(|(i, v)| field_translation[i].name.contains("departure"))
        .map(|(i, v)| v)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    const OTHER_TEST_INPUT: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    #[test]
    fn test_number_spoken() {
        assert_eq!(sum_invalid_tickets(&to_vec(TEST_INPUT)), 71);
    }

    #[test]
    fn test_departure_prod() {
        assert_eq!(departure_products(&to_vec(OTHER_TEST_INPUT)), 71);
    }
}
