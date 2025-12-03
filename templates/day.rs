use nom;

fn parse(input: &str) -> () {
    let (input, res) = todo!();
    assert!(input == "" || input == "\n");
    res
}

pub fn run(input: &str) -> String {
    format!("Part 1: {}\nPart 2 {}", run_part1(input), run_part2(input))
}

fn run_part1(input: &str) -> i64 {
    -1
}

fn run_part2(input: &str) -> i64 {
    -1
}

#[allow(dead_code)]
const TEST_INPUT: &str = "";

#[test]
fn test_part1() {
    let answer = 0;
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = 0;
    assert_eq!(answer, run_part2(TEST_INPUT));
}
