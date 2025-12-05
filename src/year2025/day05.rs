use itertools::Itertools;
use lib::span::{Span, consolidate_spans};
use macros::solution;

fn parse(input: &str) -> (Vec<Span<i64>>, Vec<i64>) {
    let mut parts = input.trim().split("\n\n");
    let fresh_ranges: Vec<Span<i64>> = parts
        .next()
        .unwrap()
        .split('\n')
        .map(|x| {
            Span::from(
                x.split('-')
                    // .inspect(|y| println!("{y}"))
                    .map(|y| y.parse::<i64>().unwrap())
                    .collect_tuple::<(i64, i64)>()
                    .unwrap(),
            )
        })
        .collect_vec();
    let ingredient_ids = parts
        .next()
        .unwrap()
        .split('\n')
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();
    (consolidate_spans(fresh_ranges), ingredient_ids)
}

#[solution(year = 2025, day = 5, part = 1)]
fn part1(input: &str) -> String {
    let (fresh_ranges, ingredient_ids) = parse(input);
    let mut sum = 0u64;
    for ingredient_id in ingredient_ids {
        if fresh_ranges
            .iter()
            .any(|span| span.is_within(ingredient_id))
        {
            sum += 1
        }
    }

    sum.to_string()
}

#[solution(year = 2025, day = 5, part = 2)]
fn part2(input: &str) -> String {
    // fresh_ranges is already de-overlapped
    let (fresh_ranges, _) = parse(input);
    fresh_ranges
        .iter()
        .map(|span| span.end - span.start + 1)
        .sum::<i64>()
        .to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

#[test]
fn test_part1() {
    let answer = "3";
    assert_eq!(answer, part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "14";
    assert_eq!(answer, part2(TEST_INPUT));
}
