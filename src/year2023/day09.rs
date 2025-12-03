use macros::solution;
use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
};

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    for line in input.lines() {
        let (_, l) = separated_list1(space1::<&str, nom::error::Error<&str>>, complete::i32)(line)
            .expect("Could not parse line");
        res.push(l);
    }
    res
}

fn extrapolate(nums: Vec<i32>, backwards: bool) -> i32 {
    if nums.len() == 1 {
        nums[0]
    } else {
        let mut extrapolated_nums = Vec::new();
        for i in 1..nums.len() {
            extrapolated_nums.push(nums[i] - nums[i - 1]);
        }
        let x = if backwards {
            nums.first().unwrap() - extrapolate(extrapolated_nums, backwards)
        } else {
            nums.last().unwrap() + extrapolate(extrapolated_nums, backwards)
        };
        //println!("{:?} {}", nums, x);
        x
    }
}

//too high: 1947980994
#[solution(year=2023, day=9, part=1)]
fn run_part1(input: &str) -> String {
    let numbers = parse(input);
    numbers
        .into_iter()
        .map(|x| extrapolate(x, false))
        .sum::<i32>()
        .to_string()
}

#[solution(year=2023, day=9, part=2)]
fn run_part2(input: &str) -> String {
    let numbers = parse(input);
    numbers
        .into_iter()
        .map(|x| extrapolate(x, true))
        .sum::<i32>()
        .to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

#[test]
fn test_part1() {
    let answer = "114";
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "2";
    assert_eq!(answer, run_part2(TEST_INPUT));
}
