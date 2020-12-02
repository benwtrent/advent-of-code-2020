fn sums_to<'a>(
    sum: &'_ usize,
    curr: &'a usize,
    rest: &'a [usize],
) -> Option<(&'a usize, &'a usize)> {
    if rest.len() == 0 {
        return Option::None;
    }
    for re in rest {
        if re + curr == *sum {
            return Option::Some((curr, re));
        }
    }
    sums_to(sum, &rest[0], &rest[1..])
}

fn tri_sums_to<'a>(
    sum: &'_ usize,
    x: &'a usize,
    rest: &'a [usize],
) -> Option<(&'a usize, &'a usize, &'a usize)> {
    if rest.len() == 0 {
        return Option::None;
    }
    if x < sum {
        let sub_sum = *sum - x;
        for i in 0..(rest.len() - 1) {
            if let Some((a, b)) = sums_to(&sub_sum, &rest[i], &rest[i + 1..]) {
                return Option::Some((x, a, b));
            }
        }
    }
    tri_sums_to(sum, &rest[0], &rest[1..])
}

#[aoc_generator(day1)]
fn input_to_vec(input: &str) -> Vec<usize> {
    input.lines().map(|i| i.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn day1_1(input: &Vec<usize>) -> usize {
    let (v1, v2) = sums_to(&2020, &input[0], &input[1..]).unwrap();
    v1 * v2
}

#[aoc(day1, part2)]
fn day1_2(input: &Vec<usize>) -> usize {
    let (v1, v2, v3) = tri_sums_to(&2020, &input[0], &input[1..]).unwrap();
    v1 * v2 * v3
}
