use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::{self, alpha1, char, newline, space1},
    multi::separated_list0,
    sequence::pair,
    IResult,
};
use substring::Substring;

#[derive(Debug)]
enum Monkey {
    Const(i32),
    Operation {
        first: String,
        second: String,
        op: Operator,
    },
}

#[derive(Debug)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

fn parse_operator_monkey(input: &str) -> IResult<&str, (&str, Monkey)> {
    let (input, (name, _)) = pair(alpha1, char(':'))(input)?;

    let (input, _) = space1(input)?;
    let (input, first) = alpha1(input)?;
    let (input, _) = space1(input)?;
    let (input, op_char) = alt((char('+'), char('-'), char('*'), char('/')))(input)?;
    let (input, _) = space1(input)?;
    let (input, second) = alpha1(input)?;

    let op = match op_char {
        '+' => Operator::Plus,
        '-' => Operator::Minus,
        '*' => Operator::Multiply,
        '/' => Operator::Divide,
        _ => panic!("not gonna happen"),
    };

    Ok((
        input,
        (
            name,
            Monkey::Operation {
                first: first.to_owned(),
                second: second.to_owned(),
                op,
            },
        ),
    ))
}

fn parse_const_monkey(input: &str) -> IResult<&str, (&str, Monkey)> {
    let (input, (name, _)) = pair(alpha1, char(':'))(input)?;

    let (input, (_, constant)) = pair(space1, complete::i32)(input)?;

    Ok((input, (name, Monkey::Const(constant))))
}

fn parse_monkeys(input: &str) -> IResult<&str, HashMap<&str, Monkey>> {
    let (input, named_monkeys) =
        separated_list0(newline, alt((parse_operator_monkey, parse_const_monkey)))(input)?;

    let mut result: HashMap<&str, Monkey> = HashMap::new();
    for (name, monkey) in named_monkeys {
        result.insert(name, monkey);
    }

    Ok((input, result))
}

fn solve_first(monkeys: &HashMap<&str, Monkey>, name: &str) -> i64 {
    let current = monkeys.get(name).expect("monkey should be present");
    match current {
        Monkey::Const(c) => *c as i64,
        Monkey::Operation { first, second, op } => {
            let first_res = solve_first(monkeys, first);
            let second_res = solve_first(monkeys, second);

            match op {
                Operator::Plus => first_res + second_res,
                Operator::Minus => first_res - second_res,
                Operator::Multiply => first_res * second_res,
                Operator::Divide => first_res / second_res,
            }
        }
    }
}

fn try_solve(monkeys: &HashMap<&str, Monkey>, name: &str) -> Option<i64> {
    let current = monkeys.get(name);

    if let Some(current) = current {
        match current {
            Monkey::Const(c) => Some(*c as i64),
            Monkey::Operation { first, second, op } => {
                let first_res = try_solve(monkeys, first);
                let second_res = try_solve(monkeys, second);

                if let (Some(first_res), Some(second_res)) = (first_res, second_res) {
                    Some(match op {
                        Operator::Plus => first_res + second_res,
                        Operator::Minus => first_res - second_res,
                        Operator::Multiply => first_res * second_res,
                        Operator::Divide => first_res / second_res,
                    })
                } else {
                    None
                }
            }
        }
    } else {
        None
    }
}

fn solve_second(monkeys: &HashMap<&str, Monkey>, name: &str, expected: i64) -> i64 {
    let current = monkeys.get(name);

    if let Some(current) = current {
        match current {
            Monkey::Const(_) => panic!("should not happen"),
            Monkey::Operation { first, second, op } => {
                let first_res = try_solve(monkeys, first);
                let second_res = try_solve(monkeys, second);

                match (first_res, second_res) {
                    (Some(first_res), None) => {
                        let next_exp = match op {
                            Operator::Plus => expected - first_res,
                            Operator::Minus => first_res - expected,
                            Operator::Multiply => expected / first_res,
                            Operator::Divide => first_res / expected,
                        };
                        solve_second(monkeys, second, next_exp)
                    }
                    (None, Some(second_res)) => {
                        let next_exp = match op {
                            Operator::Plus => expected - second_res,
                            Operator::Minus => second_res + expected,
                            Operator::Multiply => expected / second_res,
                            Operator::Divide => second_res * expected,
                        };
                        solve_second(monkeys, first, next_exp)
                    }
                    (None, None) => panic!("both depend on humn?!"),
                    (Some(_), Some(_)) => panic!("none depend on humn?!"),
                }
            }
        }
    } else {
        expected
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let (remaining, monkeys) = parse_monkeys(input).expect("should parse");
    if !remaining.is_empty() {
        panic!(
            "did not fully parse, remaining: {}...",
            input.substring(0, 20)
        );
    }

    let res = solve_first(&monkeys, "root");

    Some(res)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (remaining, mut monkeys) = parse_monkeys(input).expect("should parse");
    if !remaining.is_empty() {
        panic!(
            "did not fully parse, remaining: {}...",
            input.substring(0, 20)
        );
    }

    monkeys.remove("humn");
    let root = monkeys.get("root").expect("root should be present");

    if let Monkey::Operation {
        first,
        second,
        op: _,
    } = root
    {
        let second = solve_first(&monkeys, second);
        let first = solve_second(&monkeys, first, second);

        Some(first)
    } else {
        panic!("root should be composite");
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
