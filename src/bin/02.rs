#[derive(Debug)]
enum Play {
    Rock,
    Paper,
    Scissor,
}

use Play::*;

use std::str::FromStr;
#[derive(Debug)]
enum ParseError {
    ParsePlay,
    ParseWinner,
    ParsePair,
}

impl FromStr for Play {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissor),
            "X" => Ok(Rock),
            "Y" => Ok(Paper),
            "Z" => Ok(Scissor),
            _ => Err(ParseError::ParsePlay),
        }
    }
}

fn map_line_part1(line: &str) -> Option<(Play, Play)> {
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

use Winner::*;

impl FromStr for Winner {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(First),
            "Y" => Ok(Draw),
            "Z" => Ok(Second),
            _ => Err(ParseError::ParseWinner),
        }
    }
}

fn winner((first, second): &(Play, Play)) -> Winner {
    match (first, second) {
        (Rock, Scissor) => First,
        (Rock, Paper) => Second,
        (Paper, Rock) => First,
        (Paper, Scissor) => Second,
        (Scissor, Paper) => First,
        (Scissor, Rock) => Second,
        _ => Draw,
    }
}

fn score(t: &(Play, Play)) -> u32 {
    let result_points = match winner(t) {
        First => 0,
        Draw => 3,
        Second => 6,
    };
    let pick_points = match t.1 {
        Rock => 1,
        Paper => 2,
        Scissor => 3,
    };
    pick_points + result_points
}

pub fn part_one(input: &str) -> Option<u32> {
    let plays = input
        .split('\n')
        .map(|line| map_line_part1(line).expect("mapping line correctly"));
    let scores = plays.map(|t| score(&t));
    Some(scores.sum())
}

fn map_line_part2(line: &str) -> Result<(Play, Winner), ParseError> {
    let spl = line.split_once(' ');
    match spl {
        Some((f, l)) => {
            let f = Play::from_str(f)?;
            let l = Winner::from_str(l)?;
            Ok((f, l))
        }
        None => Err(ParseError::ParsePair),
    }
}

fn make_plays(t: &(Play, Winner)) -> (Play, Play) {
    match t {
        (Rock, First) => (Rock, Scissor),
        (Rock, Second) => (Rock, Paper),
        (Rock, Draw) => (Rock, Rock),
        (Scissor, First) => (Scissor, Paper),
        (Scissor, Second) => (Scissor, Rock),
        (Scissor, Draw) => (Scissor, Scissor),
        (Paper, First) => (Paper, Rock),
        (Paper, Second) => (Paper, Scissor),
        (Paper, Draw) => (Paper, Paper),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let plays_with_winners = input
        .split('\n')
        .map(|line| map_line_part2(line).expect("mapping line correctly"));
    let scores = plays_with_winners
        .map(|t| make_plays(&t))
        .map(|t| score(&t));
    Some(scores.sum())
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
        assert_eq!(part_two(&input), Some(12));
    }
}
