use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let input: Vec<char> = input.chars().collect();
    for i in 0..(input.len() - 3_usize) {
        let (ai, bi, ci, di) = (i, i + 1_usize, i + 2_usize, i + 3_usize);
        let (a, b, c, d) = (input[ai], input[bi], input[ci], input[di]);
        let mut hash: HashSet<char> = HashSet::new();
        hash.insert(a);
        hash.insert(b);
        hash.insert(c);
        hash.insert(d);
        if hash.len() == 4 {
            return Some(di as u32 + 1);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    const MESSAGE_PREFIX_SIZE: usize = 14;
    let input: Vec<char> = input.chars().collect();
    for i in 0..(input.len() - MESSAGE_PREFIX_SIZE - 1_usize) {
        let indices = 0..MESSAGE_PREFIX_SIZE;
        let mut hash: HashSet<char> = HashSet::new();
        for ix in indices {
            hash.insert(input[i + ix]);
        }
        if hash.len() == MESSAGE_PREFIX_SIZE {
            return Some((i + MESSAGE_PREFIX_SIZE) as u32);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_one(input), Some(7));
    }

    #[test]
    fn test_part_one_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part_one(input), Some(5));
    }

    #[test]
    fn test_part_one_3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part_one(input), Some(6));
    }

    #[test]
    fn test_part_one_4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part_one(input), Some(10));
    }

    #[test]
    fn test_part_one_5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part_one(input), Some(11));
    }

    #[test]
    fn test_part_two_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_two(input), Some(19));
    }

    #[test]
    fn test_part_two_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part_two(input), Some(23));
    }

    #[test]
    fn test_part_two_3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part_two(input), Some(23));
    }

    #[test]
    fn test_part_two_4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part_two(input), Some(29));
    }

    #[test]
    fn test_part_two_5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part_two(input), Some(26));
    }
}
