struct WorkAssignment {
    start: u32,
    end: u32,
}

impl WorkAssignment {
    pub fn fully_overlaps_either(&self, other: &Self) -> bool {
        if self.start <= other.start && self.end >= other.end {
            return true;
        }
        if other.start <= self.start && other.end >= self.end {
            return true;
        }
        false
    }
    pub fn overlaps_either(&self, other: &Self) -> bool {
        if self.start <= other.start && self.end >= other.start {
            return true;
        }
        if other.start <= self.start && other.end >= self.start {
            return true;
        }
        false
    }
}

use std::str::FromStr;
#[derive(Debug)]
enum ParseError {
    Split,
    NotInt,
    PairSplit,
}

impl FromStr for WorkAssignment {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or(ParseError::Split)?;
        let start = start.parse().map_err(|_| ParseError::NotInt)?;
        let end = end.parse().map_err(|_| ParseError::NotInt)?;
        Ok(WorkAssignment { start, end })
    }
}

fn parse_line_as_pair(line: &str) -> Result<(WorkAssignment, WorkAssignment), ParseError> {
    let (fst, snd) = line.split_once(',').ok_or(ParseError::PairSplit)?;
    let fst = WorkAssignment::from_str(fst)?;
    let snd = WorkAssignment::from_str(snd)?;
    Ok((fst, snd))
}

fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| parse_line_as_pair(line).expect("should be parseable"))
            .filter(|(w1, w2)| w1.fully_overlaps_either(w2))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| parse_line_as_pair(line).expect("should be parseable"))
            .filter(|(w1, w2)| w1.overlaps_either(w2))
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fully_contains_either_1() {
        let first = WorkAssignment { start: 1, end: 2 };
        let other = WorkAssignment { start: 3, end: 4 };
        assert_eq!(first.fully_overlaps_either(&other), false);
        assert_eq!(other.fully_overlaps_either(&first), false);
    }

    #[test]
    fn test_fully_contains_either_2() {
        let first = WorkAssignment { start: 1, end: 3 };
        let other = WorkAssignment { start: 3, end: 3 };
        assert_eq!(first.fully_overlaps_either(&other), true);
        assert_eq!(other.fully_overlaps_either(&first), true);
    }

    #[test]
    fn test_fully_contains_either_3() {
        let first = WorkAssignment { start: 2, end: 8 };
        let other = WorkAssignment { start: 3, end: 7 };
        assert_eq!(first.fully_overlaps_either(&other), true);
        assert_eq!(other.fully_overlaps_either(&first), true);
    }

    #[test]
    fn test_overlaps_either_1() {
        let first = WorkAssignment { start: 5, end: 7 };
        let other = WorkAssignment { start: 7, end: 9 };
        assert_eq!(first.overlaps_either(&other), true);
        assert_eq!(other.overlaps_either(&first), true);
    }

    #[test]
    fn test_overlaps_either_2() {
        let first = WorkAssignment { start: 1, end: 2 };
        let other = WorkAssignment { start: 3, end: 4 };
        assert_eq!(first.overlaps_either(&other), false);
        assert_eq!(other.overlaps_either(&first), false);
    }

    #[test]
    fn test_overlaps_either_3() {
        let first = WorkAssignment { start: 2, end: 8 };
        let other = WorkAssignment { start: 3, end: 7 };
        assert_eq!(first.overlaps_either(&other), true);
        assert_eq!(other.overlaps_either(&first), true);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
