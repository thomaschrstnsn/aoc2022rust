#[derive(Debug)]
enum Play {
    Rock,
    Paper,
    Scissor,
}

use std::str::FromStr;
#[derive(Debug)]
enum ParsePlayError {
    ParseErr,
}

impl FromStr for Play {
    type Err = ParsePlayError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Play::Rock),
            "B" => Ok(Play::Paper),
            "C" => Ok(Play::Scissor),
            "X" => Ok(Play::Rock),
            "Y" => Ok(Play::Paper),
            "Z" => Ok(Play::Scissor),
            _ => Err(ParsePlayError::ParseErr),
        }
    }
}

fn map_line(line: &str) -> Option<(Play, Play)> {
    let spl = line.split_once(' ');
    match spl {
        Some((f, l)) => match Play::from_str(f) {
            Ok(fp) => match Play::from_str(l) {
                Ok(lp) => Some((fp, lp)),
                Err(_) => None,
            },
            Err(_) => None,
        },
        None => None,
    }
}

#[derive(Debug)]
enum Winner {
    First,
    Second,
    Draw,
}

fn winner((first, second): &(Play, Play)) -> Winner {
    match (first, second) {
        (Play::Rock, Play::Scissor) => Winner::First,
        (Play::Rock, Play::Paper) => Winner::Second,
        (Play::Paper, Play::Rock) => Winner::First,
        (Play::Paper, Play::Scissor) => Winner::Second,
        (Play::Scissor, Play::Paper) => Winner::First,
        (Play::Scissor, Play::Rock) => Winner::Second,
        _ => Winner::Draw,
    }
}

fn score_part_one(t: &(Play, Play)) -> u32 {
    let result_points = match winner(&t) {
        Winner::First => 0,
        Winner::Draw => 3,
        Winner::Second => 6,
    };
    let pick_points = match t.1 {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissor => 3,
    };
    pick_points + result_points
}

pub fn part_one(input: &str) -> Option<u32> {
    let plays = input
        .split('\n')
        .map(|line| map_line(line).expect("mapping line correctly"));
    let scores = plays.map(|t| score_part_one(&t));
    Some(scores.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
