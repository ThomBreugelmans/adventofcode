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
        //println!("{}-{}", a, b);
        for n in a..=b {
            let char_count = f64::log10(n as f64) as u32 + 1;
            let n1 = n / 10i64.pow(char_count / 2);
            let n2 = n % 10i64.pow(char_count / 2);
            if n1 == n2 {
                sum += n;
            }
        }
    }
    format!("{}", sum)
}

#[solution(year = 2025, day = 2, part = 2)]
fn run_part2(input: &str) -> String {
    let parsed = parse(input);
    let mut sum = 0;
    for (a, b) in parsed {
        for n in a..=b {
            let char_count = f64::log10(n as f64) as u32 + 1;
            'search: for i in 1..=(char_count / 2) {
                let divisor = 10i64.pow(i);
                let n1 = n % divisor;
                if f64::log10(n1 as f64) as u32 + 1 != i {
                    // else we get stuff like n1=08, which cannot be a repeating sequence
                    continue;
                }
                let mut n2 = n;
                while n2 > 0 {
                    let remainder = n2 % divisor;
                    n2 /= divisor;
                    if remainder != n1 {
                        continue 'search;
                    }
                }
                sum += n;
                break;
            }
        }
    }
    format!("{}", sum)
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
