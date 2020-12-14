use bitvec::prelude::*;
use regex;
use std::collections::{HashMap, HashSet};
use std::ops::BitAndAssign;

struct MaskAndValues {
    mask: HashMap<u8, u8>,
    values: Vec<(usize, usize)>,
}

impl MaskAndValues {
    fn add_values(&self, totals: &mut HashMap<usize, usize>) {
        for (key, value) in &self.values {
            totals.insert(*key, self.masked_value(value));
        }
    }

    fn add_values_multiple_places(&self, totals: &mut HashMap<usize, usize>) {
        for (key, value) in &self.values {
            let keys = self.get_keys(key);
            for k in keys {
                totals.insert(k, *value);
            }
        }
    }

    fn get_keys(&self, key: &usize) -> Vec<usize> {
        let bits = key.view_bits::<Lsb0>();
        let mut bits = BitVec::from_bitslice(bits);
        for (pos, val) in &self.mask {
            if *val == 0 {
                continue;
            }
            bits.set(*pos as usize, true);
        }
        let new_key = bits.load::<usize>();
        let mut values = HashSet::new();
        for bit in 0..36usize {
            if !self.mask.contains_key(&(bit as u8)) {
                bits.set(bit, false);
            }
        }
        values.insert(bits.load::<usize>());
        MaskAndValues::masking_recur(
            &self.mask.keys().map(|v| *v).collect(),
            &mut bits,
            0,
            &mut values,
        );
        values.iter().map(|v| *v).collect()
    }

    fn masking_recur(
        mask: &HashSet<u8>,
        bits: &mut BitVec,
        curr_bit: u8,
        values: &mut HashSet<usize>,
    ) {
        if curr_bit > 35 {
            return;
        }
        let mut curr_bit = curr_bit;
        while mask.contains(&(curr_bit as u8)) {
            curr_bit += 1;
        }
        if curr_bit > 35 {
            return;
        }
        bits.set(curr_bit as usize, true);
        values.insert(bits.load::<usize>());
        MaskAndValues::masking_recur(mask, bits, curr_bit + 1, values);
        bits.set(curr_bit as usize, false);
        values.insert(bits.load::<usize>());
        MaskAndValues::masking_recur(mask, bits, curr_bit + 1, values);
    }

    fn masked_value(&self, value: &usize) -> usize {
        let bits = value.view_bits::<Lsb0>();
        let mut bits = BitVec::from_bitslice(bits);
        for (pos, val) in &self.mask {
            bits.set(*pos as usize, *val == 1);
        }
        bits.load::<usize>()
    }
}

impl From<&str> for MaskAndValues {
    fn from(s: &str) -> Self {
        let r = regex::Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
        let mut lines = s.lines();
        let mut mask = HashMap::new();
        let mut curr_bit = 36;
        for v in lines.next().unwrap().split("").filter(|s| !s.is_empty()) {
            curr_bit -= 1;
            if v != "X" {
                mask.insert(curr_bit, v.parse().unwrap());
            }
        }
        let mut values: Vec<(usize, usize)> = vec![];
        for l in lines {
            if let Some(captures) = r.captures(l) {
                values.push((captures[1].parse().unwrap(), captures[2].parse().unwrap()));
            }
        }
        MaskAndValues { mask, values }
    }
}

#[aoc_generator(day14)]
fn to_vec(input: &str) -> Vec<MaskAndValues> {
    input
        .split("mask = ")
        .filter(|s| !s.is_empty())
        .map(|i| i.into())
        .collect()
}

#[aoc(day14, part1)]
fn masked_bit_sums(input: &Vec<MaskAndValues>) -> usize {
    let mut vals: HashMap<usize, usize> = HashMap::new();
    for v in input {
        v.add_values(&mut vals);
    }
    vals.values().sum()
}

#[aoc(day14, part2)]
fn data_mask_sums(input: &Vec<MaskAndValues>) -> usize {
    let mut vals: HashMap<usize, usize> = HashMap::new();
    for v in input {
        v.add_values_multiple_places(&mut vals);
    }
    vals.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";

    const OTHER_TEST_INPUT: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";

    #[test]
    fn test_bit_sum() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(masked_bit_sums(&input), 165);
    }

    #[test]
    fn test_bit_multi_sum() {
        let input = to_vec(OTHER_TEST_INPUT);
        assert_eq!(data_mask_sums(&input), 208);
    }
}
