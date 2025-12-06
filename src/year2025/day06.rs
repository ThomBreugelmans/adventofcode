// use itertools::Itertools;
use lib::utils;
use macros::solution;
use std::iter::zip;

fn parse(mut input: &str) -> (Vec<Vec<i64>>, Vec<fn(i64, i64) -> Option<i64>>) {
    input = input.trim_matches('\n');
    let line_count = input.chars().filter(|c| *c == '\n').count() + 1;
    let mut input_iter = input.split('\n');
    let equations = input_iter
        .by_ref()
        .take(line_count - 1)
        .map(|x| {
            x.split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    let operations = input_iter
        .take(1)
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
    input = input.trim_matches('\n');
    let line_count = input.chars().filter(|c| *c == '\n').count() + 1;
    let mut input_iter = input.split('\n').peekable();
    let row_len = input_iter.peek().unwrap().len();
    let mut row_iterators: Vec<_> = input_iter
        .by_ref()
        .take(line_count - 1)
        .map(|row| row.chars())
        .collect();
    let equations = (0..row_len)
        .map(|_| {
            let n_string = String::from_iter(row_iterators.iter_mut().map(|r| r.next().unwrap()));
            n_string
        })
        // .inspect(|s| println!("{s}"))
        .map(|n_string| match n_string.trim().parse::<i64>() {
            Ok(num) => Some(num),
            Err(_) => None,
        })
        .fold(vec![Vec::new()], |mut v, n| {
            match n {
                Some(val) => v.last_mut().unwrap().push(val),
                None => v.push(Vec::new()),
            };
            v
        });

    let operations: Vec<fn(i64, i64) -> Option<i64>> = input_iter
        .take(1)
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
