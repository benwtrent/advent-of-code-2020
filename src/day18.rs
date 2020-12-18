use regex;
use std::collections::VecDeque;
use std::hint::unreachable_unchecked;

enum Token {
    Prod,
    Add,
    Num(u128),
    OpenParen,
    CloseParen,
}

fn do_math_from_str(line: &str) -> u128 {
    let split = line.split(" ");
    let mut tokens = VecDeque::new();
    let open_paren_pattern = regex::Regex::new(r"(\(+)(\d+)").unwrap();
    let close_paren_pattern = regex::Regex::new(r"(\d+)(\)+)").unwrap();
    for s in split.filter(|&s| !s.is_empty()) {
        if s.starts_with("(") {
            let captures = open_paren_pattern.captures(s).unwrap();
            for _ in 0..captures[1].len() {
                tokens.push_back(Token::OpenParen);
            }
            tokens.push_back(Token::Num(captures[2].parse().unwrap()));
        } else if s.ends_with(")") {
            let captures = close_paren_pattern.captures(s).unwrap();
            let num:u128 = captures[1].parse().unwrap();
            let paren_count:usize = captures[2].len();
            let operand = tokens.pop_back().unwrap();
            let val = tokens.pop_back().unwrap();
            if let Token::Num(other) = val {
                let mut new = match operand {
                    Token::Prod => num * other,
                    Token::Add => num + other,
                    _ => unreachable!()
                };
                let mut open_parent_count = 0;
                while open_parent_count < paren_count {
                    match tokens.pop_back().unwrap() {
                        Token::OpenParen => open_parent_count += 1,
                        Token::Prod => if let Token::Num(o) = tokens.pop_back().unwrap(){
                            new *= o;
                        },
                        Token::Add => if let Token::Num(o) = tokens.pop_back().unwrap(){
                            new += o;
                        },
                        _ => unreachable!()
                    }
                }
                if let Some(t) = tokens.back() {
                    match t {
                        Token::Prod => {
                            tokens.pop_back().unwrap();
                            if let Token::Num(o) = tokens.pop_back().unwrap() {
                                new *= o;
                            }
                        },
                        Token::Add => {
                            tokens.pop_back().unwrap();
                            if let Token::Num(o) = tokens.pop_back().unwrap() {
                                new += o;
                            }
                        },
                        _ => {}
                    }
                }
                tokens.push_back(Token::Num(new))
            };
        } else if s == "*" {
            tokens.push_back(Token::Prod)
        } else if s == "+" {
            tokens.push_back(Token::Add)
        } else { // its a number
            let v: u128 = s.parse().unwrap();
            if tokens.is_empty() {
                tokens.push_back(Token::Num(v))
            } else {
                let operand = tokens.pop_back().unwrap();
                let val = tokens.pop_back().unwrap();
                if let Token::Num(other) = val {
                    match operand {
                        Token::Prod => tokens.push_back(Token::Num(v * other)),
                        Token::Add => tokens.push_back(Token::Num(v + other)),
                        _ => unreachable!()
                    }
                }
            }
        }
    }
    if let Token::Num(val) = tokens.pop_back().unwrap() {
        let mut result = val;
        while !tokens.is_empty() {
            let operand = tokens.pop_back().unwrap();
            let val = tokens.pop_back().unwrap();
            if let Token::Num(other) = val {
                match operand {
                    Token::Prod => result *= other,
                    Token::Add => result += other,
                    _ => unreachable!()
                }
            }
        }
        return result;
    }
    0
}

fn reduce(tokens: &[Token]) -> u128 {
    let to_mul = vec![];
    
}

fn do_other_math_from_str(line: &str) -> u128 {
    let mut split = line.split(" ");
    let mut tokens = VecDeque::new();
    let open_paren_pattern = regex::Regex::new(r"(\(+)(\d+)").unwrap();
    let close_paren_pattern = regex::Regex::new(r"(\d+)(\)+)").unwrap();
    for s in split.filter(|&s| !s.is_empty()) {
        if s.starts_with("(") {
            let captures = open_paren_pattern.captures(s).unwrap();
            for _ in 0..captures[1].len() {
                tokens.push_back(Token::OpenParen);
            }
            tokens.push_back(Token::Num(captures[2].parse().unwrap()));
        } else if s.ends_with(")") {
            let captures = close_paren_pattern.captures(s).unwrap();
            let num:u128 = captures[1].parse().unwrap();
            let paren_count:usize = captures[2].len();
            if let Token::Add = tokens.back().unwrap() {
                let operand = tokens.pop_back().unwrap();
                let val = tokens.pop_back().unwrap();
                if let Token::Num(other) = val {
                    let mut new = match operand {
                        Token::Prod => num * other,
                        Token::Add => num + other,
                        _ => unreachable!()
                    };
                let operand = tokens.pop_back().unwrap();
                let val = tokens.pop_back().unwrap();
                if let Token::Num(other) = val {
                    let mut open_parent_count = 0;
                    while open_parent_count < paren_count {
                        match tokens.pop_back().unwrap() {
                            Token::OpenParen => open_parent_count += 1,
                            /*Token::Prod => if let Token::Num(o) = tokens.pop_back().unwrap(){
                                new *= o;
                            },*/
                            Token::Add => if let Token::Num(o) = tokens.pop_back().unwrap(){
                                new += o;
                            },
                            _ => unreachable!()
                        }
                    }
                    if let Some(t) = tokens.back() {
                        match t {
                            /*Token::Prod => {
                                tokens.pop_back().unwrap();
                                if let Token::Num(o) = tokens.pop_back().unwrap() {
                                    new *= o;
                                }
                            },*/
                            Token::Add => {
                                tokens.pop_back().unwrap();
                                if let Token::Num(o) = tokens.pop_back().unwrap() {
                                    new += o;
                                }
                            },
                            _ => {}
                        }
                    }
                    tokens.push_back(Token::Num(new))
                };
            }
            
        } {
                tokens.push_back(Token::Num(num))
            }
            } else if s == "*" {
            tokens.push_back(Token::Prod)
        } else if s == "+" {
            tokens.push_back(Token::Add)
        } else { // its a number
            let v: u128 = s.parse().unwrap();
            if tokens.is_empty() {
                tokens.push_back(Token::Num(v))
            } else {
                if let Token::Add = tokens.back().unwrap() {
                    let operand = tokens.pop_back().unwrap();
                    let val = tokens.pop_back().unwrap();
                    if let Token::Num(other) = val {
                        match operand {
                            Token::Add => tokens.push_back(Token::Num(v + other)),
                            _ => unreachable!()
                        }
                    }
                } else {
                    tokens.push_back(Token::Num(v));
                }
            }
        }
    }
    if let Token::Num(val) = tokens.pop_back().unwrap() {
        let mut result = val;
        while !tokens.is_empty() {
            let operand = tokens.pop_back().unwrap();
            let val = tokens.pop_back().unwrap();
            if let Token::Num(other) = val {
                match operand {
                    Token::Prod => result *= other,
                    Token::Add => result += other,
                    _ => unreachable!()
                }
            }
        }
        return result;
    }
    0
}


#[aoc(day18, part1)]
fn do_the_dumb_math(input: &str) -> u128 {
    input.lines().map(|l| do_math_from_str(l)).sum()
}


#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn test_do_math(){
        assert_eq!(do_math_from_str("2 * 3 + (4 * 5)"), 26);
        assert_eq!(do_math_from_str("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(do_math_from_str("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(do_math_from_str("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
        assert_eq!(do_math_from_str("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(do_math_from_str("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(do_math_from_str("((5 + 6 + 9 + 6) + 3 + 4 * 6 * 5)"), 990);
        assert_eq!(do_math_from_str("9 * 3 * ((3 * 6 + 4 + 8) + 9 + 2) * 3"), 3321);
    }

    #[test]
    fn test_do_other_math() {
        assert_eq!(do_other_math_from_str("2 * 3 + (4 * 5)"), 46);
        assert_eq!(do_other_math_from_str("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(do_other_math_from_str("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
        assert_eq!(do_other_math_from_str("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
        assert_eq!(do_other_math_from_str("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(do_other_math_from_str("1 + 2 * 3 + 4 * 5 + 6"), 231);
        //assert_eq!(do_other_math_from_str("((5 + 6 + 9 + 6) + 3 + 4 * 6 * 5)"), 990);
        //assert_eq!(do_other_math_from_str("9 * 3 * ((3 * 6 + 4 + 8) + 9 + 2) * 3"), 3321);
    }
    
}
