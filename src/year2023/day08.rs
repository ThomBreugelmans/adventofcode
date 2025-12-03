use macros::solution;
use std::{collections::BTreeMap, iter::Cycle, vec::IntoIter};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    sequence::{preceded, separated_pair, terminated},
};
use num::Integer;

#[derive(Copy, Clone, Debug)]
enum Dir {
    Left,
    Right,
}
type Instructions<T> = Cycle<IntoIter<T>>;

fn parse_instructions(input: &str) -> (&str, Instructions<Dir>) {
    let (res, matched) = terminated(alpha1::<&str, nom::error::Error<&str>>, tag("\n\n"))(input)
        .expect("could not parse instructions");
    let mut instructions = Vec::new();
    for c in matched.chars() {
        if c == 'L' {
            instructions.push(Dir::Left);
        } else if c == 'R' {
            instructions.push(Dir::Right);
        }
    }
    (res, instructions.into_iter().cycle())
}

fn parse_paths(input: &str) -> BTreeMap<&str, (&str, &str)> {
    let mut paths = BTreeMap::new();
    for line in input.lines() {
        let (_, (key, val)) = separated_pair(
            alphanumeric1::<&str, nom::error::Error<&str>>,
            tag(" = "),
            preceded(
                tag("("),
                separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            ),
        )(line)
        .expect("could not parse directions");
        paths.insert(key, val);
    }
    paths
}

fn parse(input: &str) -> (Instructions<Dir>, BTreeMap<&str, (&str, &str)>) {
    let (input, instructions) = parse_instructions(input);

    let paths = parse_paths(input);
    (instructions, paths)
}

#[solution(year = 2023, day = 8, part = 1)]
fn run_part1(input: &str) -> String {
    let (mut instructions, paths) = parse(input);
    let mut cur_pos = "AAA";
    let mut count = 0;
    while let Some(instruction) = instructions.next() {
        cur_pos = match instruction {
            Dir::Left => paths.get(cur_pos).unwrap().0,
            Dir::Right => paths.get(cur_pos).unwrap().1,
        };
        count += 1;
        if cur_pos == "ZZZ" {
            break;
        }
    }
    count.to_string()
}

#[solution(year = 2023, day = 8, part = 2)]
fn run_part2(input: &str) -> String {
    let (instructions, paths) = parse(input);
    let positions = paths
        .keys()
        .clone()
        .filter(|s| s.ends_with('A'))
        .collect::<Vec<_>>();

    positions
        .into_iter()
        .map(|mut x| {
            let mut instructions_ = instructions.clone().enumerate();
            while let Some((i, instruction)) = instructions_.next() {
                x = match instruction {
                    Dir::Left => &paths.get(x).unwrap().0,
                    Dir::Right => &paths.get(x).unwrap().1,
                };

                if x.ends_with('Z') {
                    return i + 1;
                }
            }
            unreachable!()
        })
        .reduce(|a, b| a.lcm(&b))
        .unwrap()
        .to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

#[allow(dead_code)]
const TEST_INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

#[allow(dead_code)]
const TEST_INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

#[test]
fn test_part1() {
    let answer = "2";
    assert_eq!(answer, run_part1(TEST_INPUT));
    let answer2 = "6";
    assert_eq!(answer2, run_part1(TEST_INPUT2));
}

#[test]
fn test_part2() {
    let answer = "6";
    assert_eq!(answer, run_part2(TEST_INPUT3));
}
