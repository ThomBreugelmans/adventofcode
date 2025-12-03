use itertools::Itertools;
use macros::solution;

fn parse(input: &str) -> Vec<(char, i64)> {
    input
        .trim()
        .split('\n')
        .map(|r| r.chars().collect_vec())
        .map(|x| {
            (
                x[0],
                x[1..]
                    .iter()
                    .rfold(0i64, |r, &v| r * 10 + (v as u8 - '0' as u8) as i64),
            )
        })
        .collect()
}

#[solution(year = 2025, day = 1, part = 1)]
pub fn run_part1(input: &str) -> String {
    let parsed = parse(input);
    let mut pos = 50i64;
    let mut sum = 0;
    for (d, count) in parsed {
        let di = if d == 'L' { -1 } else { 1 };
        let new_pos = pos + (di * count);
        if new_pos.rem_euclid(100) == 0 {
            sum += 1;
        }
        pos = new_pos;
    }
    format!("{}", sum)
}

#[solution(year = 2025, day = 1, part = 2)]
pub fn run_part2(input: &str) -> String {
    let parsed = parse(input);
    let mut pos = 50i64;
    let mut sum = 0;
    for (d, count) in parsed {
        let di = if d == 'L' { -1 } else { 1 };
        let new_pos = pos + (di * count);
        let nearest_100 = if pos % 100 == 0 {
            pos + (100 * di)
        } else {
            ((pos as f64 / 100f64) + if d == 'L' { -1f64 } else { 0f64 }).ceil() as i64 * 100
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
    format!("{}", sum)
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
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "6";
    assert_eq!(answer, run_part2(TEST_INPUT));
}
