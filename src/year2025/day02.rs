use itertools::Itertools;
use macros::solution;

fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .split(',')
        .map(|p| {
            p.split('-')
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

#[solution(year = 2025, day = 2, part = 1)]
fn run_part1(input: &str) -> String {
    let parsed = parse(input);
    let mut sum = 0;
    for (a, b) in parsed {
        let char_count_a = i64::ilog10(a) as u32 + 1;
        let half_pow_a = 10i64.pow(char_count_a / 2 + if char_count_a % 2 == 0 { 0 } else { 1 });
        let half_pow_b = 10i64.pow((i64::ilog10(b) as u32 + 1) / 2);
        let n1 = (a / half_pow_a).max(1);
        let n2 = b / half_pow_b;
        for n_ in n1..=n2 {
            let pow = i64::ilog10(n_) as u32 + 1;
            let n = n_ * 10i64.pow(pow) + n_;
            if (a..=b).contains(&n) {
                sum += n;
            }
        }
    }
    sum.to_string()
}

#[solution(year = 2025, day = 2, part = 2)]
fn run_part2(input: &str) -> String {
    let parsed = parse(input);
    let mut found = Vec::new();
    for (a, b) in parsed {
        let char_count_a = i64::ilog10(a) as u32 + 1;
        let char_count_b = i64::ilog10(b) as u32 + 1;
        let start = (a / (10i64.pow(char_count_a))).max(1);
        let end = b / 10i64.pow(char_count_b / 2);
        for n in start..=end {
            let pow_n = 10i64.pow(i64::ilog10(n) as u32 + 1);
            let mut m = n;
            while m < a {
                m = m * pow_n + n;
            }
            while (a..=b).contains(&m) {
                if !found.contains(&m) {
                    found.push(m);
                }
                m = m * pow_n + n;
            }
        }
    }
    found.into_iter().sum::<i64>().to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

#[test]
fn test_part1() {
    let answer = "1227775554";
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "4174379265";
    assert_eq!(answer, run_part2(TEST_INPUT));
}
