#[derive(Debug, Clone)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

use nom::{
    branch::alt,
    character::complete::{self, newline},
    multi::separated_list0,
    sequence::{delimited, pair, separated_pair},
    IResult,
};
use substring::Substring;
use Packet::*;

fn parse_integer(input: &str) -> IResult<&str, Packet> {
    let (input, value) = complete::u32(input)?;
    Ok((input, Integer(value)))
}

fn parse_list(input: &str) -> IResult<&str, Packet> {
    let (input, packets) = delimited(
        complete::char('['),
        separated_list0(complete::char(','), parse_packet),
        complete::char(']'),
    )(input)?;
    Ok((input, List(packets)))
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((parse_list, parse_integer))(input)
}

fn parse_packet_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
    separated_pair(parse_packet, newline, parse_packet)(input)
}

fn parse_packet_pairs(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list0(pair(newline, newline), parse_packet_pair)(input)
}

fn are_lists_in_right_order(left: &[Packet], right: &[Packet]) -> Option<bool> {
    for (index, left) in left.iter().enumerate() {
        match right.get(index) {
            None => {
                return Some(false);
            }
            Some(right) => {
                let decision = is_in_right_order(left, right);
                if decision.is_some() {
                    return decision;
                }
            }
        }
    }
    if left.len() == right.len() {
        return None;
    }
    Some(true)
}

fn is_in_right_order(left: &Packet, right: &Packet) -> Option<bool> {
    match (left, right) {
        (Integer(left), Integer(right)) => {
            if left == right {
                None
            } else {
                Some(left < right)
            }
        }
        (List(left), List(right)) => are_lists_in_right_order(left, right),
        (Integer(_), List(right)) => are_lists_in_right_order(&[left.clone()], right),
        (List(left), Integer(_)) => are_lists_in_right_order(left, &[right.clone()]),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (remaining, packets) = parse_packet_pairs(input).expect("parses correctly");
    if !remaining.is_empty() {
        panic!(
            "did not fully parse, remaining: {}...",
            input.substring(0, 20)
        );
    }

    let result: Vec<u32> = packets
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| is_in_right_order(left, right).unwrap_or(true))
        .map(|(index, _)| (index as u32) + 1)
        .collect();

    dbg!(&result);

    Some(result.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_in_right_order_example_4() {
        // [[4,4],4,4] vs [[4,4],4,4,4]
        let left = List(vec![
            List(vec![Integer(4), Integer(4)]),
            Integer(4),
            Integer(4),
        ]);
        let right = List(vec![
            List(vec![Integer(4), Integer(4)]),
            Integer(4),
            Integer(4),
            Integer(4),
        ]);
        assert_eq!(is_in_right_order(&left, &right), Some(true))
    }

    #[test]
    fn test_is_in_the_right_order_example_8_false() {
        let input = r#"[[],[9],[4,[[10]],8,10,[10,10,[],[]]],[[],[[10,4,6]],[[1,1,6],[]],5],[[[1,7,5],[10,1,6],6,[]],[],2,3,9]]
[[],[[4,[5,4,8,7],[10]]],[10,7,[3],8],[[6,[1,2,9,5]],[],[[2,4,3]],[3,[3,8,9,8],[9]]],[[[6,0,0,7,3],9,3],[9,[0,4]],[[8,8],[2,1,8],[]],3,[]]]"#;
        let (remaining, pairs) = parse_packet_pairs(input).expect("parses");
        assert!(remaining.is_empty());
        let (left, right) = &pairs[0];
        assert_eq!(is_in_right_order(left, right), Some(false));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
