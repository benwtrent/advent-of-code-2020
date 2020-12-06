use std::collections::HashSet;

#[aoc_generator(day6)]
fn to_vec(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|i| {
            i.lines()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect()
        })
        .collect()
}

#[aoc(day6, part1)]
fn answer_count(input: &Vec<Vec<Vec<char>>>) -> usize {
    input
        .iter()
        .map(|group| {
            group
                .iter()
                .flat_map(|v| v.iter().map(|c| *c).collect::<HashSet<char>>())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
fn abs_answer_count(input: &Vec<Vec<Vec<char>>>) -> usize {
    input
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|v| v.iter().map(|c| *c).collect::<HashSet<char>>())
                .fold(Option::None, |l, g| {
                    if l.is_none() {
                        Some(g.clone())
                    } else {
                        Some(
                            l.unwrap_or(HashSet::new())
                                .intersection(&g)
                                .map(|c| *c)
                                .collect(),
                        )
                    }
                })
                .unwrap_or(HashSet::new())
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";
    #[test]
    fn test_answered_yes_count() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(answer_count(&input), 11);
    }

    #[test]
    fn test_abs_answered_yes_count() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(abs_answer_count(&input), 6);
    }
}
