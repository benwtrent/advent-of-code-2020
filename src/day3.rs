enum Entry {
    Tree,
    Snow
}

impl From<&str> for Entry {
    fn from(s: &str) -> Self {
        match s { 
            "." => Entry::Snow,
            "#" => Entry::Tree,
            _ => panic!(format!("unexpected string {}", s))
        }
    }
}

#[aoc_generator(day3)]
fn input_to_vec(input: &str) -> Vec<Vec<Entry>> {
    input.lines().map(|i| {
        let splt = i.split("").filter(|s| !s.is_empty()).map(|s| Entry::from(s)).collect();
        splt
    }).collect()
}

fn tree_count_for_steps(input: &Vec<Vec<Entry>>, x: usize, y: usize) -> usize {
    let mut ct = 0;
    let mut right = 0;
    for r in 1..input.len() {
        if r % y > 0 {
            continue;
        }
        let row = &input[r];
        right += x;
        right %= row.len();
        if let Entry::Tree = row[right] {
            ct += 1;
        }
    }
    ct
}

#[aoc(day3, part1)]
fn tree_count(input: &Vec<Vec<Entry>>) -> usize {
    let mut ct = 0;
    let mut x = 0;
    for r in 1..input.len() {
        let row = &input[r];
        x += 3;
        x %= row.len();
        if let Entry::Tree = row[x] {
            ct += 1;
        }
    }
    ct 
}

#[aoc(day3, part2)]
fn tree_count_for_all_paths(input: &Vec<Vec<Entry>>) -> usize {
    let mut ct = 1;
    for (x, y) in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].into_iter() {
        ct *= tree_count_for_steps(input, x, y);
    }
    ct
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_tree_count() {
        let input = input_to_vec(TEST_INPUT);
        assert_eq!(tree_count(&input), 7);
    }

    #[test]
    fn test_tree_count_for_steps() {
        let input = input_to_vec(TEST_INPUT);
        assert_eq!(tree_count_for_steps(&input, 3, 1), 7);
    }
    
    #[test]
    fn test_tree_count_all_paths() {
        let input = input_to_vec(TEST_INPUT);
        assert_eq!(tree_count_for_all_paths(&input), 336);
    }


}