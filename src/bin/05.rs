use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar, digit1, multispace1, newline, space1},
    multi::{count, many1, separated_list1},
    sequence::{pair, preceded, terminated},
    IResult,
};

type SupplyStack = Vec<char>;

fn parse_single_create(input: &str) -> IResult<&str, Option<char>> {
    let (input, _) = complete::char('[')(input)?;
    let (input, c) = anychar(input)?;
    let (input, _) = complete::char(']')(input)?;
    Ok((input, Some(c)))
}

fn parse_empty_stack_elm(input: &str) -> IResult<&str, Option<char>> {
    let (input, _) = tag("   ")(input)?;
    Ok((input, None))
}

fn parse_crates(input: &str) -> IResult<&str, Vec<SupplyStack>> {
    let (input, horizontal_crates) = separated_list1(
        newline,
        separated_list1(tag(" "), alt((parse_empty_stack_elm, parse_single_create))),
    )(input)?;
    let (input, _) = newline(input)?;
    let (input, _nums) = many1(preceded(space1, digit1))(input)?;
    let (input, _) = multispace1(input)?;

    let mut result_crates: Vec<SupplyStack> = vec![];
    for _ in horizontal_crates.iter() {
        result_crates.push(vec![])
    }
    for vec in horizontal_crates.iter().rev() {
        for (i, c) in vec.iter().enumerate() {
            if let Some(c) = c {
                result_crates[i].push(*c)
            }
        }
    }
    Ok((input, result_crates))
}

#[derive(Debug, PartialEq)]
struct Move {
    count: u32,
    from: u32,
    to: u32,
}

fn parse_single_move(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, count) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;
    Ok((input, Move { count, from, to }))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(newline, parse_single_move)(input)
}

fn parse_puzzle(input: &str) -> IResult<&str, (Vec<SupplyStack>, Vec<Move>)> {
    pair(parse_crates, parse_moves)(input)
}

pub fn part_one(input: &str) -> Option<String> {
    None
}

pub fn part_two(input: &str) -> Option<String> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_parse_crates() {
        let input = advent_of_code::read_file("examples", 5)
            .lines()
            .take(5)
            .join("\n");
        let parsed = parse_crates(&input);
        assert_eq!(
            parsed,
            Ok(("", vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]))
        )
    }

    #[test]
    fn test_parse_moves() {
        let input = advent_of_code::read_file("examples", 5)
            .lines()
            .skip(5)
            .join("\n");
        let parsed = parse_moves(&input);
        assert_eq!(
            parsed,
            Ok((
                "",
                vec![
                    Move {
                        count: 1,
                        from: 2,
                        to: 1
                    },
                    Move {
                        count: 3,
                        from: 1,
                        to: 3
                    },
                    Move {
                        count: 2,
                        from: 2,
                        to: 1
                    },
                    Move {
                        count: 1,
                        from: 1,
                        to: 2
                    }
                ]
            ))
        )
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
