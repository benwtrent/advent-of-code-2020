#[derive(Debug)]
struct Passport {
    birth_year: Option<usize>,
    issue_year: Option<usize>,
    exp_year: Option<usize>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    pid: Option<String>,
    cid: Option<usize>,
}

impl From<&str> for Passport {
    fn from(s: &str) -> Self {
        let (
            mut birth_year,
            mut issue_year,
            mut exp_year,
            mut height,
            mut hair_color,
            mut eye_color,
            mut pid,
            mut cid,
        ) = (
            Option::None,
            Option::None,
            Option::None,
            Option::None,
            Option::None,
            Option::None,
            Option::None,
            Option::None,
        );
        s.split(" ").filter(|i| !i.is_empty()).for_each(|i| {
            let name_var: Vec<&str> = i.split(":").collect();
            match name_var[0] {
                "byr" => birth_year = Option::Some(name_var[1].parse().unwrap()),
                "iyr" => issue_year = Option::Some(name_var[1].parse().unwrap()),
                "eyr" => exp_year = Option::Some(name_var[1].parse().unwrap()),
                "hgt" => height = Option::Some(String::from(name_var[1])),
                "hcl" => hair_color = Option::Some(String::from(name_var[1])),
                "ecl" => eye_color = Option::Some(String::from(name_var[1])),
                "pid" => pid = Option::Some(String::from(name_var[1])),
                "cid" => cid = Option::Some(name_var[1].parse().unwrap()),
                _ => {}
            }
        });
        Passport {
            birth_year,
            issue_year,
            exp_year,
            height,
            hair_color,
            eye_color,
            pid,
            cid,
        }
    }
}

impl Passport {
    pub fn is_valid(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.exp_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.pid.is_some()
    }

    pub fn is_valid_strict(&self) -> bool {
        self.valid_birth_year()
            && self.valid_issue_year()
            && self.valid_exp_year()
            && self.valid_hgt()
            && self.valid_hair()
            && self.valid_eyes()
            && self.valid_pid()
    }

    fn valid_birth_year(&self) -> bool {
        (1920..=2002).contains(&self.birth_year.unwrap_or_default())
    }

    fn valid_issue_year(&self) -> bool {
        (2010..=2020).contains(&self.issue_year.unwrap_or_default())
    }

    fn valid_exp_year(&self) -> bool {
        (2020..=2030).contains(&self.exp_year.unwrap_or_default())
    }

    fn valid_hgt(&self) -> bool {
        if let Some(height) = self.height.as_ref() {
            let range = match &height[height.len() - 2..] {
                "in" => (59..=76),
                "cm" => (150..=193),
                _ => return false,
            };
            range.contains(&height[0..height.len() - 2].parse::<usize>().unwrap_or(0))
        } else {
            false
        }
    }

    fn valid_hair(&self) -> bool {
        Passport::valid_str(self.hair_color.as_ref(), r"^#[0-9a-f]{6}$")
    }

    fn valid_eyes(&self) -> bool {
        Passport::valid_str(self.eye_color.as_ref(), r"^amb|blu|brn|gry|grn|hzl|oth$")
    }

    fn valid_pid(&self) -> bool {
        Passport::valid_str(self.pid.as_ref(), r"^[0-9]{9}$")
    }

    fn valid_str(maybe_str: Option<&String>, re: &str) -> bool {
        if let Some(str) = maybe_str {
            let re = regex::Regex::new(re).unwrap();
            let captures = re.captures(str.as_str());
            captures.is_some()
        } else {
            false
        }
    }
}

#[aoc_generator(day4)]
fn input_to_vec(input: &str) -> Vec<Passport> {
    let mut cleaned_str = String::from("");
    let mut cleaned_input: Vec<Passport> = vec![];
    input.lines().for_each(|i| {
        if i.is_empty() && !cleaned_str.is_empty() {
            cleaned_input.push(Passport::from(cleaned_str.as_str()));
            cleaned_str = String::from("");
        }
        cleaned_str += i;
        cleaned_str += " ";
    });
    if !cleaned_str.is_empty() {
        cleaned_input.push(Passport::from(cleaned_str.as_str()));
    }
    cleaned_input
}

#[aoc(day4, part1)]
fn valid_count(input: &Vec<Passport>) -> usize {
    input.iter().filter(|p| p.is_valid()).count()
}

#[aoc(day4, part2)]
fn strict_valid_count(input: &Vec<Passport>) -> usize {
    input.iter().filter(|p| p.is_valid_strict()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_valid_count() {
        let input = input_to_vec(TEST_INPUT);
        assert_eq!(valid_count(&input), 2);
    }

    #[test]
    fn test_strict_valid_count() {
        let valids = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let invalids = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let input = input_to_vec(valids);
        assert_eq!(strict_valid_count(&input), 4);
        let input = input_to_vec(TEST_INPUT);
        assert_eq!(strict_valid_count(&input), 2);
        let input = input_to_vec(invalids);
        assert_eq!(strict_valid_count(&input), 0);
    }

    fn valid_passport() -> Passport {
        Passport {
            birth_year: Option::Some(2000),
            issue_year: Option::Some(2010),
            exp_year: Option::Some(2021),
            height: Option::Some(String::from("155cm")),
            hair_color: Option::Some(String::from("#123abc")),
            eye_color: Option::Some(String::from("brn")),
            pid: Option::Some(String::from("000000001")),
            cid: Option::None,
        }
    }

    #[test]
    fn test_individual_valids() {
        {
            let p = Passport {
                height: Option::Some(String::from("190")),
                ..valid_passport()
            };
            assert!(!p.is_valid_strict());
        }
        {
            let p = Passport {
                pid: Option::Some(String::from("0123456789")),
                ..valid_passport()
            };
            assert!(!p.is_valid_strict());
        }
    }
}
