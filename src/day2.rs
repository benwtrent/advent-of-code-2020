#[derive(Debug, Eq, PartialEq)]
pub struct PasswordPolicy {
    character: char,
    min: usize,
    max: usize,
} 

impl From<&String> for PasswordPolicy {
    fn from(s: &String) -> Self {
        let spaces:Vec<&str> = s.split(" ").collect();
        let character:char = spaces[1].chars().next().unwrap();
        let vec:Vec<usize> = spaces[0].split("-").map(|i| i.parse().unwrap()).collect();
        PasswordPolicy {
            character,
            min: vec[0],
            max: vec[1]
        }
    }
}


impl PasswordPolicy {
    
    pub fn satisfied_1(&self, password: &str) -> bool {
        let ct = password.chars().filter(|c| *c == self.character).count();
        (self.min - 1) < ct && ct < (self.max + 1)
    }

    pub fn satisfied_2(&self, password: &str) -> bool {
        let mut ans = false;
        for (i, c) in password.chars().enumerate() {
            if (i + 1) == self.min  || (i + 1) == self.max {
                ans ^= self.character == c;
            }
        }
        ans
    }
}

#[aoc_generator(day2)]
fn input_to_vec(input: &str) -> Vec<(PasswordPolicy, String)> {
    input.lines().map(|i| {
        let splt = i.split(": ").map(|s| String::from(s)).collect::<Vec<String>>();
        (PasswordPolicy::from(&splt[0]), splt[1].to_string())
    }).collect()
}

#[aoc(day2, part1)]
fn valid_password_count(input: &Vec<(PasswordPolicy, String)>) -> usize {
    input.iter().filter(|(policy, password)| policy.satisfied_1(password.as_str())).count()
}

#[aoc(day2, part2)]
fn valid_password_count2(input: &Vec<(PasswordPolicy, String)>) -> usize {
    input.iter().filter(|(policy, password)| policy.satisfied_2(password.as_str())).count()
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn valid_policy() {
        let input = "1-3 a: abcde \n1-3 b: cdefg \n2-9 c: ccccccccc\n";
        let input = input_to_vec(input);
        assert_eq!(valid_password_count(&input), 2)
    }

    #[test]
    fn valid_policy_2() {
        let input = "1-3 a: abcde \n1-3 b: cdefg \n2-9 c: ccccccccc\n";
        let input = input_to_vec(input);
        assert_eq!(valid_password_count2(&input), 1)
    }
    
}