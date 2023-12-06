use std::iter::zip;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1, space1},
    multi::separated_list1,
    sequence::preceded,
};

fn parse_(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (input, time_) = preceded(
        tag("Time:"),
        preceded(
            space1::<&str, nom::error::Error<&str>>,
            separated_list1(multispace1, digit1),
        ),
    )(input)
    .unwrap();
    let (_, distance_) = preceded(
        tag("\nDistance:"),
        preceded(
            space1::<&str, nom::error::Error<&str>>,
            separated_list1(multispace1, digit1),
        ),
    )(input)
    .unwrap();
    (time_, distance_)
}

fn parse(input: &str) -> Vec<(u32, u32)> {
    let (time_, distance_) = parse_(input);
    zip(time_, distance_)
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .collect::<Vec<(u32, u32)>>()
}

fn parse2(input: &str) -> (u64, u64) {
    let (time, distance) = parse_(input);
    (
        time.join("").parse::<u64>().unwrap(),
        distance.join("").parse::<u64>().unwrap(),
    )
}

pub fn run(input: &str) -> String {
    format!("Part 1: {}\nPart 2: {}", run_part1(input), run_part2(input))
}

fn run_part1(input: &str) -> String {
    let games = parse(input);

    games
        .iter()
        .map(|(time, record)| {
            (0..*time)
                .filter_map(|speed| (speed * (time - speed) > *record).then_some(speed))
                .count()
        })
        .product::<usize>()
        .to_string()
}

fn run_part2(input: &str) -> String {
    let (time, record) = parse2(input);
    (0..time)
        .filter(|speed| speed * (time - speed) > record)
        .count()
        .to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

#[test]
fn test_part1() {
    let answer = "288";
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "71503";
    assert_eq!(answer, run_part2(TEST_INPUT));
}
