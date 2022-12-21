use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list0,
    IResult,
};
use substring::Substring;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
    fn length(&self) -> i32 {
        self.dist(&Point { x: 0, y: 0 })
    }
}

// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
#[derive(Debug)]
struct SensorAndClosestBeacon {
    sensor: Point,
    closest_beacon: Point,
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, _) = tag("x=")(input)?;
    let (input, x) = complete::i32(input)?;

    let (input, _) = tag(", y=")(input)?;
    let (input, y) = complete::i32(input)?;

    Ok((input, Point { x, y }))
}

fn parse_single_input(input: &str) -> IResult<&str, SensorAndClosestBeacon> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, sensor) = parse_point(input)?;
    let (input, _) = tag(": closest beacon is at ")(input)?;
    let (input, closest_beacon) = parse_point(input)?;
    Ok((
        input,
        SensorAndClosestBeacon {
            sensor,
            closest_beacon,
        },
    ))
}

fn parse_inputs(input: &str) -> IResult<&str, Vec<SensorAndClosestBeacon>> {
    separated_list0(newline, parse_single_input)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (remaining, packets) = parse_inputs(input).expect("parses correctly");
    if !remaining.is_empty() {
        panic!(
            "did not fully parse, remaining: {}...",
            input.substring(0, 20)
        );
    }
    dbg!(&packets);

    let y_row = if packets.len() > 15 { 2_000_000 } else { 10 };

    let min_x = packets
        .iter()
        .map(|p| p.sensor.x - p.sensor.dist(&p.closest_beacon) - 2)
        .min()
        .expect("should");
    let max_x = packets
        .iter()
        .map(|p| p.sensor.x + p.sensor.dist(&p.closest_beacon) + 1)
        .max()
        .expect("should");

    dbg!(&(min_x, max_x));

    let mut points_that_cannot_have_beacon: HashSet<Point> = HashSet::new();
    for x in min_x..max_x {
        let point = Point { x, y: y_row };
        for p in &packets {
            if p.sensor.dist(&point) <= p.sensor.dist(&p.closest_beacon) {
                points_that_cannot_have_beacon.insert(point.clone());
                continue;
            }
        }
    }
    for p in packets {
        points_that_cannot_have_beacon.remove(&p.closest_beacon);
    }

    Some(points_that_cannot_have_beacon.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
