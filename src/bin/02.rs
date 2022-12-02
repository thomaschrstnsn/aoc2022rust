#[derive(Debug)]
enum Play {
    Rock,
    Paper,
    Scissor
}


use std::str::FromStr;
#[derive(Debug)]
enum ParsePlayError {
        ParseErr
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

pub fn part_one(input: &str) -> Option<u32> {
    let plays : Vec<Play> = input.split('\n')
        .map(|s| s.split_once(' ').map(|(x,y)| Play::from_str(x)))
        .collect();
    Some(32)
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
