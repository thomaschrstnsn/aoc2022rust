use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
enum Instruction {
    NoOp,
    AddX(i32),
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;

    Ok((input, Instruction::NoOp))
}
fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("addx ")(input)?;
    let (input, num) = complete::i32(input)?;

    Ok((input, Instruction::AddX(num)))
}

fn parse_program(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, alt((parse_noop, parse_addx)))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (remaining, program) = parse_program(input).expect("parses");
    if !remaining.is_empty() {
        panic!("did not fully parse, remaining: {}", input);
    }
    let mut reg_x: i32 = 1;
    let mut cycle: u32 = 1;
    let mut signal_strengths: Vec<i32> = Vec::new();
    let cycle_counts_of_interest: HashSet<u32> =
        HashSet::from_iter(vec![20, 60, 100, 140, 180, 220]);
    for i in program {
        let (cycle_inc, reg_x_delta) = match i {
            Instruction::NoOp => (1, 0),
            Instruction::AddX(d) => (2, d),
        };
        let cycle_next = cycle + cycle_inc;
        for c in cycle..cycle_next {
            if cycle_counts_of_interest.contains(&c) {
                signal_strengths.push(c as i32 * reg_x);
            }
        }
        cycle = cycle_next;
        reg_x += reg_x_delta;
    }

    dbg!(&signal_strengths);
    Some(signal_strengths.iter().sum::<i32>() as u32)
}

struct State {
    reg_x: i32,
    cycle: u32,
    current_line: Vec<char>,
}

impl State {
    pub fn new() -> Self {
        State {
            reg_x: 1,
            cycle: 0,
            current_line: Vec::new(),
        }
    }

    pub fn draw_pixel(&mut self) -> Option<String> {
        let pixel = match self.cycle as i32 - self.reg_x {
            -1 | 0 | 1 => '#',
            _ => '.',
        };
        self.current_line.push(pixel);

        self.cycle += 1;
        if self.cycle == 40 {
            let line: String = self.current_line.iter().collect();
            self.current_line = Vec::new();
            self.cycle = 0;
            return Some(line);
        }
        None
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let (remaining, program) = parse_program(input).expect("parses");
    if !remaining.is_empty() {
        panic!("did not fully parse, remaining: {}", input);
    }
    let mut lines: Vec<String> = Vec::new();
    let mut state = State::new();

    for i in program {
        let (cycle_inc, reg_x_delta) = match i {
            Instruction::NoOp => (1, 0),
            Instruction::AddX(d) => (2, d),
        };
        for _i in 0..cycle_inc {
            let finished_line = state.draw_pixel();
            if let Some(finished_line) = finished_line {
                lines.push(finished_line);
            }
        }
        state.reg_x += reg_x_delta;
    }

    Some(lines.join("\n"))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        let expected: String = r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#
            .to_owned();
        dbg!(&expected.lines().collect::<Vec<&str>>());
        assert_eq!(part_two(&input), Some(expected));
    }
}
