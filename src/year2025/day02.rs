use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::separated_pair
};

fn parse(input: &str) -> Vec<(i64,i64)> {
    let (inp, res) = separated_list1(
        tag(","),
        separated_pair(
            complete::i64::<&str, nom::error::Error<&str>>,
            tag("-"),
            complete::i64
        )
    )(input).unwrap();
    assert!(inp == "" || inp == "\n");
    res
}

pub fn run(input: &str) -> String {
    format!("Part 1: {}\nPart 2: {}", run_part1(input), run_part2(input))
}

fn run_part1(input: &str) -> i64 {
    let parsed = parse(input);
    let mut sum = 0;
    for (a, b) in parsed {
        //println!("{}-{}", a, b);
        for n in a..=b {
            let str_n = format!("{}", n);
            if str_n.len() % 2 != 0 {
                continue;
            }
            let half = str_n.len() / 2;
            let (n1, n2) = str_n.split_at(half);
            // let n2 = &str_n[half..str_n.len()];

            if n1 == n2 {
                sum += n;
            }
        }
    }
    sum
}

fn run_part2(input: &str) -> i64 {
    let parsed = parse(input);
    let mut sum = 0;
    for (a, b) in parsed {
        //println!("{}-{}", a, b);
        for n in a..=b {
            let str_n = format!("{}", n);
            let half = str_n.len() / 2;
            for i in 1..=half {
                let (n1, n2) = str_n.split_at(i);
                let mut pref = n1.to_string();
                while pref.len() < n2.len() {
                    pref = format!("{}{}", pref, n1);
                }
                if pref == n2 {
                    // println!("{}", n);
                    sum += n;
                    break;
                }
            }
        }
    }
    sum
}

#[allow(dead_code)]
const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

#[test]
fn test_part1() {
    let answer = 1227775554;
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = 4174379265;
    assert_eq!(answer, run_part2(TEST_INPUT));
}
