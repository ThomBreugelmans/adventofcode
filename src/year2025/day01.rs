use macros::solution;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete,
    combinator::opt,
    multi::separated_list1,
    sequence::{preceded, terminated},
};

fn parse_right(input: &str) -> IResult<&str, i64> {
    preceded(nom::character::char('R'), complete::i64).parse(input)
}

fn parse_left(input: &str) -> IResult<&str, i64> {
    preceded(nom::character::char('L'), complete::i64.map(|n| -n)).parse(input)
}

fn parse(input: &str) -> Vec<i64> {
    let (_, res) = terminated(
        separated_list1(nom::character::char('\n'), alt((parse_right, parse_left))),
        opt(nom::character::char('\n')),
    )
    .parse_complete(input.trim())
    .unwrap();
    res
}

#[solution(year = 2025, day = 1, part = 1)]
pub fn part1(input: &str) -> String {
    let parsed = parse(input);
    let mut pos = 50i64;
    let mut sum = 0;
    for ticks in parsed {
        let new_pos = pos + ticks;
        if new_pos.rem_euclid(100) == 0 {
            sum += 1;
        }
        pos = new_pos;
    }
    sum.to_string()
}

#[solution(year = 2025, day = 1, part = 2)]
pub fn part2(input: &str) -> String {
    let parsed = parse(input);
    let mut pos = 50i64;
    let mut sum = 0;
    for ticks in parsed {
        let d = ticks / ticks.abs();
        let new_pos = pos + ticks;
        let nearest_100 = if pos % 100 == 0 {
            pos + (100 * d)
        } else {
            ((pos as f64 / 100f64) + if d < 0 { -1f64 } else { 0f64 }).ceil() as i64 * 100
        };
        let dist_nearest = (nearest_100 - pos).abs();
        let dist = (new_pos - pos).abs();
        sum += if dist >= dist_nearest {
            (dist - dist_nearest).abs() / 100 + 1
        } else {
            0
        };

        pos = new_pos.rem_euclid(100);
    }
    sum.to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

#[test]
fn test_part1() {
    let answer = "3";
    assert_eq!(answer, part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "6";
    assert_eq!(answer, part2(TEST_INPUT));
}
