use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
struct BagRule {
    num: usize,
    bag_type: String,
}

impl BagRule {
    fn contains_recur(&self, bag: &str, collection: &HashMap<String, Vec<BagRule>>) -> bool {
        if self.bag_type == bag {
            return true;
        }
        collection
            .get(&self.bag_type)
            .unwrap()
            .iter()
            .any(|br| br.contains_recur(bag, collection))
    }

    fn bag_count(&self, collection: &HashMap<String, Vec<BagRule>>, prev_count: usize) -> usize {
        let rules = collection.get(&self.bag_type).unwrap();
        if rules.is_empty() {
            prev_count
        } else {
            rules
                .iter()
                .map(|br| br.bag_count(collection, br.num * prev_count))
                .sum::<usize>()
                + prev_count
        }
    }
}

impl From<&str> for BagRule {
    fn from(s: &str) -> Self {
        match s.find(" ") {
            Some(n) => {
                let num: usize = s[0..n].parse().unwrap();
                BagRule {
                    num,
                    bag_type: String::from(s[n + 1..].trim_end_matches("s")),
                }
            }
            // no bags
            None => {
                panic!("boom")
            }
        }
    }
}

#[aoc_generator(day7)]
fn to_hashmap(input: &str) -> HashMap<String, Vec<BagRule>> {
    input
        .lines()
        .map(|i| {
            let mut splt = i.split(" contain ");
            let bag = splt.next().unwrap().trim_end_matches("s");
            let unparsed_rules = splt.next().unwrap().trim_end_matches(".");
            let rules: Vec<BagRule> = if unparsed_rules == "no other bags" {
                vec![]
            } else {
                unparsed_rules.split(", ").map(|s| s.into()).collect()
            };
            (String::from(bag), rules)
        })
        .collect()
}

#[aoc(day7, part1)]
fn how_many_shiny_gold(input: &HashMap<String, Vec<BagRule>>) -> usize {
    input
        .iter()
        .filter(|(bag, rules)| {
            if bag.as_str() == "shiny gold bag" {
                false
            } else {
                rules
                    .iter()
                    .any(|br| br.contains_recur("shiny gold bag", input))
            }
        })
        .count()
}

#[aoc(day7, part2)]
fn how_many_in_shiny_gold(input: &HashMap<String, Vec<BagRule>>) -> usize {
    let rules = input.get("shiny gold bag").unwrap();
    return rules
        .iter()
        .map(|br| br.bag_count(input, br.num))
        .sum::<usize>();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const OTHER_INPUT: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
    #[test]
    fn test_how_many_bags() {
        let input = to_hashmap(TEST_INPUT);
        assert_eq!(how_many_shiny_gold(&input), 4);
    }

    #[test]
    fn test_how_many_in_shiny_recur() {
        let input = to_hashmap(TEST_INPUT);
        assert_eq!(how_many_in_shiny_gold(&input), 32);
        let input = to_hashmap(OTHER_INPUT);
        assert_eq!(how_many_in_shiny_gold(&input), 126);
    }
}
