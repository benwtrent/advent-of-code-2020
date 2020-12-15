use std::collections::HashMap;

#[aoc_generator(day15)]
fn to_vec(input: &str) -> Vec<usize> {
    input
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|i| i.parse().unwrap())
        .collect()
}

fn last_spoken(input: &Vec<usize>, last: usize) -> usize {
    let mut turns_spoken: HashMap<usize, usize> = input
        .iter()
        .take(input.len() - 1)
        .enumerate()
        .map(|(i, x)| (*x, i))
        .collect();
    let mut last_spoken = *input.last().unwrap();
    for i in input.len()..last {
        let maybe_spoken = turns_spoken.get(&last_spoken);
        let newly_spoken = match maybe_spoken {
            Some(last_time) => i - *last_time - 1,
            None => 0,
        };
        turns_spoken.insert(last_spoken, i - 1);
        last_spoken = newly_spoken as usize;
    }
    return last_spoken;
}

#[aoc(day15, part1)]
fn number_spoken(input: &Vec<usize>) -> usize {
    last_spoken(input, 2020)
}

#[aoc(day15, part2)]
fn number_spoken_big(input: &Vec<usize>) -> usize {
    last_spoken(input, 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_spoken() {
        assert_eq!(number_spoken(&to_vec("0,3,6")), 436);
        assert_eq!(number_spoken(&to_vec("1,3,2")), 1);
        assert_eq!(number_spoken(&to_vec("1,2,3")), 27);
    }

    #[test]
    fn test_number_spoken_big() {
        assert_eq!(number_spoken_big(&to_vec("0,3,6")), 175594);
    }
}
