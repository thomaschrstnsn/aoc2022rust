use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar, digit1, multispace1, newline, space1},
    multi::{count, many1, separated_list1},
    sequence::{preceded, terminated},
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
