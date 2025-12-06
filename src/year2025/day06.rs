use lib::utils;
use macros::solution;
use std::iter::zip;

fn parse(mut input: &str) -> (Vec<Vec<i64>>, Vec<fn(i64, i64) -> Option<i64>>) {
    input = input.trim();
    let line_count = input.chars().filter(|c| *c == '\n').count() + 1;
    let equations = input
        .split('\n')
        .take(line_count - 1)
        .map(|x| {
            x.split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    let operations = input
        .split('\n')
        .skip(line_count - 1)
        .flat_map(|l| {
            l.split_whitespace().map(|o| match o {
                "+" => i64::checked_add,
                "*" => i64::checked_mul,
                op => panic!("Unexpected operation in puzzle input: {}", op),
            })
        })
        .collect();
    (utils::transpose(equations), operations)
}

fn parse2(mut input: &str) -> (Vec<Vec<i64>>, Vec<fn(i64, i64) -> Option<i64>>) {
    input = input.trim_ascii_end();
    let line_count = input.chars().filter(|c| *c == '\n').count() + 1;
    let equations = utils::transpose(
        input
            .split('\n')
            .take(line_count - 1)
            .map(|r| r.chars().collect())
            .collect(),
    )
    .into_iter()
    .map(|r| String::from_iter(r).trim().to_string())
    // .inspect(|r| println!("{r}"))
    .fold(vec![Vec::new()], |mut v, n| {
        match n.parse::<i64>() {
            Ok(number) => v.last_mut().unwrap().push(number),
            Err(_) => v.push(Vec::new()),
        };
        v
    });
    let operations: Vec<fn(i64, i64) -> Option<i64>> = input
        .split('\n')
        .skip(line_count - 1)
        // .inspect(|x| println!("{x}"))
        .flat_map(|l| {
            l.split_whitespace().map(|o| match o {
                "+" => i64::checked_add,
                "*" => i64::checked_mul,
                op => panic!("Unexpected operation in puzzle input: {}", op),
            })
        })
        .collect();
    (equations, operations)
}

#[solution(year = 2025, day = 6, part = 1)]
fn part1(input: &str) -> String {
    let (equations, operations) = parse(input);
    let mut sum = 0i64;
    for (equation, operation) in zip(equations, operations) {
        sum += equation
            .into_iter()
            .reduce(|a, b| operation(a, b).expect("i64 was not enough for operation"))
            .expect("Equation input was empty");
    }
    sum.to_string()
}

#[solution(year = 2025, day = 6, part = 2)]
fn part2(input: &str) -> String {
    let (equations, operations) = parse2(input);
    let mut sum = 0i64;
    for (equation, operation) in zip(equations, operations) {
        // println!("{:?}", equation);
        let res = equation
            .into_iter()
            .reduce(|a, b| operation(a, b).expect("i64 was not enough for operation"))
            .expect("Equation input was empty");
        // println!("{res}");
        sum += res;
    }
    sum.to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

#[test]
fn test_part1() {
    let answer = "4277556";
    assert_eq!(answer, part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "3263827";
    assert_eq!(answer, part2(TEST_INPUT));
}
