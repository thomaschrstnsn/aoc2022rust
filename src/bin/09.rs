use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline, one_of, space1},
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
enum Motion {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

fn parse_motion(input: &str) -> IResult<&str, Motion> {
    let (input, dir) = one_of("UDLR")(input)?;
    let (input, _) = space1(input)?;
    let (input, amount) = complete::u32(input)?;
    let motion = match dir {
        'L' => Motion::Left(amount),
        'R' => Motion::Right(amount),
        'U' => Motion::Up(amount),
        'D' => Motion::Down(amount),
        _ => panic!("unhandled direction"),
    };
    Ok((input, motion))
}

fn parse_motions(input: &str) -> IResult<&str, Vec<Motion>> {
    separated_list1(newline, parse_motion)(input)
}

type TwoD = (i32, i32);

trait TwoDExt {
    fn add(&mut self, other: &Self);
}

impl TwoDExt for TwoD {
    fn add(&mut self, other: &TwoD) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

fn head_moves(motion: &Motion) -> Vec<TwoD> {
    let (dir, repeat) = match motion {
        Motion::Up(r) => ((0, 1), *r),
        Motion::Down(r) => ((0, -1), *r),
        Motion::Left(r) => ((-1, 0), *r),
        Motion::Right(r) => ((1, 0), *r),
    };
    let mut res = Vec::new();
    for _r in 0..repeat {
        res.push(dir);
    }
    res
}

fn tail_move(tail: &mut TwoD, head: &TwoD) {
    let delta_0 = head.0 - tail.0;
    if delta_0.abs() > 2 {
        panic!("head and tail moved too far from each other");
    }
    let delta_1 = head.1 - tail.1;
    if delta_1.abs() > 2 {
        panic!("head and tail moved too far from each other");
    }
    let diagonal = delta_0.abs() + delta_1.abs() > 2;
    if diagonal {
        if delta_0.is_positive() {
            tail.0 += 1;
        }
        if delta_0.is_negative() {
            tail.0 += -1;
        }
        if delta_1.is_positive() {
            tail.1 += 1;
        }
        if delta_1.is_negative() {
            tail.1 += -1;
        }
        return;
    }
    if delta_0.is_positive() && delta_0.abs() == 2 {
        tail.0 += 1;
    }
    if delta_0.is_negative() && delta_0.abs() == 2 {
        tail.0 += -1;
    }
    if delta_1.is_positive() && delta_1.abs() == 2 {
        tail.1 += 1;
    }
    if delta_1.is_negative() && delta_1.abs() == 2 {
        tail.1 += -1;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (remaining, motions) = parse_motions(input).expect("parses");
    if !remaining.is_empty() {
        panic!("did not fully parse, remaining: {}", input);
    }
    let mut tail_visits = Vec::new();
    let mut tail = (0, 0);
    let mut head = (0, 0);
    tail_visits.push(tail);

    for motion in motions {
        for head_move in head_moves(&motion) {
            head.add(&head_move);

            tail_move(&mut tail, &head);
            tail_visits.push(tail);
        }
    }

    let visited = tail_visits.iter().unique().count();

    Some(visited as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tail_move_horizontal() {
        let mut tail = (0, 0);
        let head = (2, 0);
        tail_move(&mut tail, &head);
        assert_eq!(tail, (1, 0));
    }

    #[test]
    fn test_tail_move_horizontal_not_diagonal() {
        let mut tail = (3, 0);
        let head = (4, 1);
        tail_move(&mut tail, &head);
        assert_eq!(tail, (3, 0));
    }

    #[test]
    fn test_tail_move_diagonal_2() {
        let mut tail = (2, 3);
        let head = (4, 3);
        tail_move(&mut tail, &head);
        assert_eq!(tail, (3, 3));
    }

    #[test]
    fn test_tail_move_diagonal_3() {
        let mut tail = (4, 3);
        let head = (2, 4);
        tail_move(&mut tail, &head);
        assert_eq!(tail, (3, 4));
    }

    #[test]
    fn test_tail_move_vertical() {
        let mut tail = (0, 0);
        let head = (0, 2);
        tail_move(&mut tail, &head);
        assert_eq!(tail, (0, 1));
    }

    #[test]
    fn test_tail_move_diagonal() {
        let mut tail = (0, 0);
        let head = (1, 2);
        tail_move(&mut tail, &head);
        assert_eq!(tail, (1, 1));
    }

    #[test]
    fn test_tail_move_nothing() {
        let mut tail = (0, 0);
        let head = (1, 0);
        tail_move(&mut tail, &head);
        assert_eq!(tail, (0, 0));
    }

    #[test]
    fn test_tail_move_nothing_2() {
        let mut tail = (2, 4);
        let head = (1, 3);
        tail_move(&mut tail, &head);
        assert_eq!(tail, (2, 4));
    }

    #[test]
    fn test_tail_move_nothing_3() {
        let mut tail = (2, 4);
        let head = (2, 3);
        tail_move(&mut tail, &head);
        assert_eq!(tail, (2, 4));
    }

    #[test]
    fn test_tail_move_nothing_4() {
        let mut tail = (2, 4);
        let head = (3, 3);
        tail_move(&mut tail, &head);
        assert_eq!(tail, (2, 4));
    }

    #[test]
    fn test_head_moves() {
        assert_eq!(
            head_moves(&Motion::Right(4)),
            vec![(1, 0), (1, 0), (1, 0), (1, 0)]
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
