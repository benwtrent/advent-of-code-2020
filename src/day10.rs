use std::collections::HashMap;

#[aoc_generator(day10)]
fn to_vec(input: &str) -> Vec<usize> {
    let vec: Vec<usize> = input.lines().map(|i| i.parse().unwrap()).collect();
    let max = vec.iter().max().unwrap();
    let mut new_vec = [&[0], vec.as_slice(), &[*max + 3]].concat();
    new_vec.sort();
    new_vec
}

#[aoc(day10, part1)]
fn the_path(input: &Vec<usize>) -> usize {
    let mut one_count = 0;
    let mut three_count = 0;
    for (i, v) in input[..input.len() - 1].iter().enumerate() {
        let diff = ((*v) as i32 - (input[i + 1]) as i32).abs() as usize;
        if diff == 1 {
            one_count += 1;
        } else if diff == 3 {
            three_count += 1;
        }
    }
    one_count * three_count
}

#[aoc(day10, part2)]
fn all_combinations(input: &Vec<usize>) -> usize {
    let mut the_ways = HashMap::new();
    // Only one way to get to 0 or 1
    the_ways.insert(0, 1);
    the_ways.insert(1, 1);
    for &v in &input[2..] {
        let mut val = the_ways.get(&(v - 1)).unwrap_or(&0) + the_ways.get(&(v - 2)).unwrap_or(&0);
        if v > 2 {
            val += the_ways.get(&(v - 3)).unwrap_or(&0);
        }
        the_ways.insert(v, val);
    }
    *the_ways.get(input.last().unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const MOAR_TEST_INPUT: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_differences() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(the_path(&input), 35);

        let input = to_vec(MOAR_TEST_INPUT);
        assert_eq!(the_path(&input), 220);
    }

    #[test]
    fn test_all_paths() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(all_combinations(&input), 8);

        let input = to_vec(MOAR_TEST_INPUT);
        assert_eq!(all_combinations(&input), 19208);
    }
}
