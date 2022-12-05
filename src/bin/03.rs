fn score(c: char) -> u32 {
    if ('a'..='z').contains(&c) {
        return c as u32 - 96;
    }
    if ('A'..='Z').contains(&c) {
        return c as u32 - 64 + 26;
    }
    0
}

use std::collections::HashSet;

fn str_as_hashset(s: &str) -> HashSet<char> {
    let mut hs = HashSet::new();
    for n in s.chars() {
        hs.insert(n);
    }
    hs
}

fn overlaps(s1: &str, s2: &str) -> Option<char> {
    let h1 = str_as_hashset(s1);
    let h2 = str_as_hashset(s2);
    let mut overlaps = h1.intersection(&h2);
    overlaps.next().copied()
}

use substring::Substring;
pub fn split_in_half(input: &str) -> (&str, &str) {
    let l = input.len();
    let first = input.substring(0, l / 2);
    let secnd = input.substring(l / 2, l);
    (first, secnd)
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n');
    let splits = lines.map(|line| split_in_half(&line));
    let overlaps = splits.map(|(s1, s2)| overlaps(s1, s2).expect("should have overlap"));
    let scores = overlaps.map(score);
    Some(scores.sum())
}

use itertools::Itertools;
pub fn part_two(input: &str) -> Option<u32> {
    let scores = input
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            let a_set = str_as_hashset(a);
            let b_set = str_as_hashset(b);
            let c_set = str_as_hashset(c);
            let overlap1: HashSet<char> = a_set.intersection(&b_set).copied().collect();
            let mut overlap2 = overlap1.intersection(&c_set);
            overlap2.next().copied().expect("should have one overlap")
        })
        .map(score);
    Some(scores.sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(score('a'), 1);
        assert_eq!(score('z'), 26);
        assert_eq!(score('A'), 27);
        assert_eq!(score('Z'), 52);
    }

    #[test]
    fn test_overlaps() {
        assert_eq!(overlaps("vJrwpWtwJgWr", "hcsFMMfFFhFp"), Some('p'));
    }

    #[test]
    fn test_split_in_half() {
        assert_eq!(split_in_half("abcd"), ("ab", "cd"));
        assert_eq!(split_in_half("ab"), ("a", "b"));
        assert_eq!(split_in_half("1234567890"), ("12345", "67890"));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
