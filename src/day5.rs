fn is_lower(input: &str) -> bool {
    &input[0..1] == "F" || &input[0..1] == "L"
}

fn calculate_pos(input: &str, lower: usize, upper: usize) -> usize {
    if input.len() == 1 {
        if is_lower(input) {
            lower
        } else {
            upper
        }
    } else {
        let (lower, upper) = if is_lower(input) {
            (lower, (upper + lower) / 2)
        } else {
            ((upper + lower) / 2 + 1, upper)
        };
        calculate_pos(&input[1..], lower, upper)
    }
}

fn calc_boarding_pass(input: &str) -> usize {
    calculate_pos(&input[0..7], 0, 127) * 8 + calculate_pos(&input[input.len() - 3..], 0, 7)
}

#[aoc(day5, part1)]
fn max_boarding_pass(input: &str) -> usize {
    input
        .lines()
        .map(|s| calc_boarding_pass(s))
        .max()
        .unwrap_or(0)
}

#[aoc(day5, part2)]
fn boarding_passes(input: &str) -> usize {
    let mut v: Vec<usize> = input.lines().map(|s| calc_boarding_pass(s)).collect();
    v.sort();
    for (l, r) in v.iter().zip(v[0]..v[v.len() - 1]) {
        if *l != r {
            return r;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boarding_pass() {
        assert_eq!(calc_boarding_pass("FBFBBFFRLR"), 357);
        assert_eq!(calc_boarding_pass("FFFBBBFRRR"), 119);
        assert_eq!(calc_boarding_pass("BBFFBBFRLL"), 820);
    }
}
