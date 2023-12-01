use std::collections::VecDeque;

const DIGITS: [&[u8]; 10] = [
    b"zero", b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn first_and_last_numbers(line: &str) -> u32 {
    let first = {
        let mut l = line.as_bytes();
        'outer: loop {
            if l[0].is_ascii_digit() {
                break 'outer (l[0] - b'0') as u32;
            }
            for (v, digit) in DIGITS.iter().enumerate() {
                if l.starts_with(digit) {
                    break 'outer v as u32;
                }
            }
            l = &l[1..];
        }
    };

    let last = {
        let mut l = line.as_bytes();
        'outer: loop {
            if l[l.len() - 1].is_ascii_digit() {
                break 'outer (l[l.len() - 1] - b'0') as u32;
            }
            for (v, digit) in DIGITS.iter().enumerate() {
                if l.ends_with(digit) {
                    break 'outer v as u32;
                }
            }
            l = &l[..l.len() - 1];
        }
    };

    first * 10 + last
}

pub fn run(input: &str) -> String {
    format!("Part 1: {}\nPart 2: {}", run_part1(input), run_part2(input))
}

fn run_part1(input: &str) -> String {
    let mut res = 0;
    for line in input.lines() {
        let mut nums = VecDeque::new();
        for c in line.trim().chars() {
            if c.is_numeric() {
                nums.push_back(c);
            }
        }
        res += [*nums.front().unwrap(), *nums.back().unwrap()]
            .into_iter()
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
    }

    res.to_string()
}

fn run_part2(input: &str) -> String {
    let mut res = 0;
    for line in input.lines() {
        let x = first_and_last_numbers(line.trim());
        res += x;
    }
    res.to_string()
}

const TEST_INPUT_ONE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

const TEST_INPUT_TWO: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

#[test]
fn test_part1() {
    let answer = "142";
    assert_eq!(answer, run_part1(TEST_INPUT_ONE));
}

#[test]
fn test_part2() {
    let answer = "281";
    assert_eq!(answer, run_part2(TEST_INPUT_TWO));
}
