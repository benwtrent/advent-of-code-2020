#[aoc_generator(day9)]
fn to_vec(input: &str) -> Vec<usize> {
    input.lines().map(|i| i.parse().unwrap()).collect()
}

fn two_number_sum(desired_sum: &usize, preamble: &usize, numbers: &[usize]) -> bool {
    if numbers.len() < *preamble {
        return false;
    }
    for (i, val) in numbers[0..*preamble - 1].iter().enumerate() {
        for other_val in numbers[i + 1..*preamble].iter() {
            if val + other_val == *desired_sum {
                return true;
            }
        }
    }
    return false;
}

fn first_without_sum(preamble: &usize, input: &[usize]) -> usize {
    for i in 0..input.len() - preamble {
        if !two_number_sum(&input[i + preamble].clone(), &preamble, &input[i..]) {
            return input[i + preamble];
        }
    }
    return 0;
}

fn contiguous_set_sum(val: &usize, input: &[usize]) -> usize {
    for (i, v) in input.iter().enumerate() {
        let mut sum = *v;
        let mut vals = vec![v];
        for other_val in input[i + 1..].iter() {
            vals.push(other_val);
            sum += other_val;
            if sum == *val {
                return *vals.iter().min().unwrap() + *vals.iter().max().unwrap();
            }
            if sum > *val {
                break;
            }
        }
    }
    return 0;
}

#[aoc(day9, part1)]
fn last_value_before_rerun(input: &Vec<usize>) -> usize {
    first_without_sum(&25, &input[..])
}

#[aoc(day9, part2)]
fn code_break(input: &Vec<usize>) -> usize {
    let desired_sum = first_without_sum(&25, &input[..]);
    contiguous_set_sum(&desired_sum, &input[..])
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_first_without_sum() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(first_without_sum(&5, &input[..]), 127);
    }

    #[test]
    fn test_contiguous_set_sum() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(contiguous_set_sum(&127, &input[..]), 62);
    }
}
