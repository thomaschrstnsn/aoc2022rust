use std::{collections::HashMap, str::FromStr};

use camino::Utf8PathBuf;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::{self, line_ending, newline, space1},
    combinator::opt,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Clone)]
enum Listing {
    Dir(String),
    File { size: u32, name: String },
}

#[derive(Debug)]
enum Command {
    Ls(Vec<Listing>),
    Cd(String),
}

fn is_newline(c: char) -> bool {
    c == '\n'
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = take_till1(is_newline)(input)?;
    let (input, _) = newline(input)?;
    Ok((input, Command::Cd(dir.to_owned())))
}

fn parse_dir(input: &str) -> IResult<&str, Listing> {
    let (input, _) = tag("dir ")(input)?;
    let (input, dir) = take_till1(is_newline)(input)?;
    Ok((input, Listing::Dir(dir.to_owned())))
}

fn parse_file(input: &str) -> IResult<&str, Listing> {
    let (input, size) = complete::u32(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = take_till1(is_newline)(input)?;
    Ok((
        input,
        Listing::File {
            size,
            name: name.to_owned(),
        },
    ))
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, listings) = separated_list1(line_ending, alt((parse_file, parse_dir)))(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, Command::Ls(listings)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Command>> {
    many1(alt((parse_cd, parse_ls)))(input)
}

#[derive(Debug, Clone)]
struct Directory {
    path: Utf8PathBuf,
    listings: Vec<Listing>,
    local_dir_size: u32,
}

fn calc_local_dir_size(listings: &Vec<Listing>) -> u32 {
    listings
        .iter()
        .map(|l| match l {
            Listing::Dir(_) => 0,
            Listing::File { size, name: _ } => *size,
        })
        .sum()
}

fn run_part_one(commands: Vec<Command>) -> Vec<Directory> {
    let mut cwd = Utf8PathBuf::new();
    let mut result: Vec<Directory> = Vec::new();
    for cmd in commands {
        match cmd {
            Command::Ls(listings) => {
                let local_dir_size = calc_local_dir_size(&listings);
                let dir = Directory {
                    path: cwd.clone(),
                    listings,
                    local_dir_size,
                };
                dbg!(&dir);
                result.push(dir);
            }
            Command::Cd(dir) => {
                dbg!(&dir);
                match dir.as_str() {
                    ".." => {
                        cwd.pop();
                    }
                    p => {
                        cwd.push(p);
                    }
                };
                dbg!(&cwd);
            }
        }
    }
    result
}

fn calc_combined(
    dir: &Directory,
    fs: &HashMap<Utf8PathBuf, Directory>,
    calced: &HashMap<Utf8PathBuf, u32>,
) -> u32 {
    let subdir_size: u32 = dir
        .listings
        .iter()
        .filter_map(|d| match d {
            Listing::Dir(s) => {
                let mut next_dir_path = dir.path.clone();
                next_dir_path.push(s);
                let next_dir = fs.get(&next_dir_path).expect("can find dir in fs");
                Some(calc_combined(next_dir, fs, calced))
            }
            _ => None,
        })
        .sum();
    dir.local_dir_size + subdir_size
}

fn calc_combined_sizes(dirs: Vec<Directory>) -> HashMap<Utf8PathBuf, u32> {
    let fs: HashMap<Utf8PathBuf, Directory> =
        dirs.iter().map(|d| (d.path.clone(), d.clone())).collect();
    let mut result: HashMap<Utf8PathBuf, u32> = HashMap::new();

    for d in dirs
        .iter()
        .sorted_by_key(|d| d.path.ancestors().count())
        .rev()
    {
        let res = calc_combined(d, &fs, &result);
        result.insert(d.path.clone(), res);
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let (input, commands) = parse_input(input).expect("parsed correctly");
    if !input.is_empty() {
        panic!("did not fully parse, remaining: {}", input);
    }
    let result = run_part_one(commands);

    let combined = calc_combined_sizes(result);

    Some(
        combined
            .iter()
            .filter_map(|(_p, size)| if size < &100_000 { Some(*size) } else { None })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (input, commands) = parse_input(input).expect("parsed correctly");
    if !input.is_empty() {
        panic!("did not fully parse, remaining: {}", input);
    }
    let result = run_part_one(commands);

    let combined = calc_combined_sizes(result);

    const DISK_SIZE: u32 = 70_000_000;
    const UPDATE_SIZE: u32 = 30_000_000;

    let root_dir = Utf8PathBuf::from_str("/").expect("root is path");
    let used: u32 = combined
        .iter()
        .filter_map(|(path, size)| if path == &root_dir { Some(size) } else { None })
        .sum();
    let free = DISK_SIZE - used;
    let needed = UPDATE_SIZE - free;
    dbg!((used, free, needed));

    Some(
        combined
            .iter()
            .filter(|(_p, size)| *size > &needed)
            .sorted_by_key(|(_, size)| **size)
            .next()
            .expect("should have one mathing criteria")
            .1
            .to_owned(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cd() {
        let input = "$ cd /\n";
        println!("input: '{}'", &input);
        match parse_cd(input) {
            Ok((input, parsed)) => {
                assert_eq!(input.len(), 0);
                dbg!(parsed);
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }

    #[test]
    fn test_file() {
        let input = "12345 luis.txt";
        println!("input: '{}'", &input);
        match parse_file(input) {
            Ok((input, parsed)) => {
                assert_eq!(input.len(), 0);
                dbg!(parsed);
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }

    #[test]
    fn test_ls() {
        let input = "$ ls\ndir a\n12345 luis.txt";
        println!("input: '{}'", &input);
        match parse_ls(input) {
            Ok((input, parsed)) => {
                assert_eq!(input.len(), 0);
                dbg!(parsed);
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }

    #[test]
    fn test_parser() {
        let input = advent_of_code::read_file("examples", 7);
        match parse_input(&input) {
            Ok((input, parsed)) => {
                dbg!(input);
                assert_eq!(input.len(), 0);
                dbg!(parsed);
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
