use std::collections::HashSet;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
enum ActionType {
    Acc,
    Jmp,
    Nop,
}

impl From<&str> for ActionType {
    fn from(s: &str) -> Self {
        match s {
            "acc" => ActionType::Acc,
            "jmp" => ActionType::Jmp,
            "nop" => ActionType::Nop,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
struct Action {
    num: i32,
    action: ActionType,
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        let mut splt = s.trim().split(" ");
        let action: ActionType = splt.next().unwrap().into();
        let num: i32 = splt.next().unwrap().parse().unwrap();
        Action { num, action }
    }
}

impl Action {
    fn run(&self, pos: &i32, accumulator: &i32) -> (i32, i32) {
        match self.action {
            ActionType::Acc => (pos + 1, accumulator + self.num),
            ActionType::Jmp => (pos + self.num, accumulator.clone()),
            ActionType::Nop => (pos + 1, accumulator.clone()),
        }
    }
}

#[aoc_generator(day8)]
fn to_vec(input: &str) -> Vec<Action> {
    input.lines().map(|i| i.into()).collect()
}

fn run_actions(input: &[Action]) -> (i32, bool, Vec<usize>) {
    let mut actions_taken = HashSet::new();
    let mut action_order = vec![];
    let mut curr_action: usize = 0;
    let mut acc: i32 = 0;
    loop {
        let action = match input.get(curr_action) {
            Some(a) => a,
            None => return (acc, true, action_order),
        };
        let (new_action, new_acc) = action.run(&(curr_action as i32), &acc);
        action_order.push(new_action as usize);
        if actions_taken.contains(&new_action) {
            return (acc, false, action_order);
        }
        curr_action = new_action as usize;
        acc = new_acc;
        actions_taken.insert(new_action);
    }
}

#[aoc(day8, part1)]
fn last_value_before_rerun(input: &Vec<Action>) -> i32 {
    run_actions(input.as_slice()).0
}

#[aoc(day8, part2)]
fn fix_program(input: &Vec<Action>) -> i32 {
    let (_, _, mut action_order) = run_actions(input.as_slice());
    action_order.reverse();
    for action in action_order
        .iter()
        .filter(|a| input.get(**a).unwrap().action != ActionType::Acc)
    {
        let to_swap: &Action = input.get(*action).unwrap();
        let swapped = match to_swap.action {
            ActionType::Nop => Action {
                num: to_swap.num,
                action: ActionType::Jmp,
            },
            ActionType::Jmp => Action {
                num: to_swap.num,
                action: ActionType::Nop,
            },
            _ => unreachable!(),
        };
        let (acc, finished, _) = run_actions(
            [&input[0..*action], &[swapped], &input[*action + 1..]]
                .concat()
                .as_slice(),
        );
        if finished {
            return acc;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_last_action() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(last_value_before_rerun(&input), 5);
    }

    #[test]
    fn test_fix_run() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(fix_program(&input), 8);
    }
}
