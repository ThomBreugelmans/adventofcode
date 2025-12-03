use macros::solution;
use nom;

fn parse(input: &str) -> Vec<&str> {
    input.split('\n').map(|x| x).collect()
}

#[solution(year=202x, day=x, part=1)]
fn run_part1(input: &str) -> String {
    let parsed = parse(input);
    let mut sum = 0u64;
    sum.to_string()
}

#[solution(year=202x, day=x, part=2)]
fn run_part2(input: &str) -> String {
    let parsed = parse(input);
    let mut sum = 0u64;
    sum.to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "";

#[test]
fn test_part1() {
    let answer = "";
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "";
    assert_eq!(answer, run_part2(TEST_INPUT));
}
