use macros::solution;

fn parse(input: &str) -> Vec<&str> {
    input.trim().split('\n').map(|x| x).collect()
}

// #[solution(year=2025, day=11, part=1)]
fn part1(input: &str) -> String {
    let parsed = parse(input);
    let mut sum = 0u64;
    sum.to_string()
}

// #[solution(year=2025, day=11, part=2)]
fn part2(input: &str) -> String {
    let parsed = parse(input);
    let mut sum = 0u64;
    sum.to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "";

#[test]
fn test_part1() {
    let answer = "";
    assert_eq!(answer, part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "";
    assert_eq!(answer, part2(TEST_INPUT));
}
