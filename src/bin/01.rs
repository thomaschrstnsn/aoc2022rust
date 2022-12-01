pub fn part_one(input: &str) -> Option<u32> {
    let xs = input.split('\n').map(|s| s.parse::<u32>());
    let mut max: u32 = 0;
    let mut sum: u32 = 0;
    for x in xs {
        if let Ok(xv) = x {
            sum += xv
        } else {
            if sum > max {
                max = sum;
            }
            sum = 0;
        }
    }
    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let xs = input.split('\n').map(|s| s.parse::<u32>());
    let mut sum = 0;
    let mut vec = Vec::new();
    for x in xs {
        if let Ok(xv) = x {
            sum += xv
        } else {
            vec.push(sum);
            sum = 0;
        }
    }
    vec.sort_unstable();
    Some(vec.iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        let expected: Option<u32> = Some(24000);
        assert_eq!(part_one(&input), expected);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
