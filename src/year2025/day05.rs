use itertools::Itertools;
use macros::solution;

fn remove_overlapping(ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut total_ranges = Vec::new();
    for (a, b) in ranges {
        let colliding_ranges = total_ranges
            .iter()
            .enumerate()
            .filter(|(_, (c, d))| !(a > *d || b < *c))
            .map(|(i, (c, d))| (i, (*c, *d)))
            .collect_vec();
        if colliding_ranges.is_empty() {
            total_ranges.push((a, b));
            continue;
        }
        for i in colliding_ranges.iter().rev().map(|(i, _)| i) {
            total_ranges.remove(*i);
        }
        let min = a.min(colliding_ranges.iter().map(|(_, (a, _))| *a).min().unwrap());
        let max = b.max(colliding_ranges.iter().map(|(_, (_, b))| *b).max().unwrap());
        total_ranges.push((min, max));
    }
    total_ranges
}

fn parse(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut parts = input.trim().split("\n\n");
    let fresh_ranges = parts
        .next()
        .unwrap()
        .split('\n')
        .map(|x| {
            x.split('-')
                // .inspect(|y| println!("{y}"))
                .map(|y| y.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();
    let ingredient_ids = parts
        .next()
        .unwrap()
        .split('\n')
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();
    (remove_overlapping(fresh_ranges), ingredient_ids)
}

#[solution(year = 2025, day = 5, part = 1)]
fn part1(input: &str) -> String {
    let (fresh_ranges, ingredient_ids) = parse(input);
    let mut sum = 0u64;
    for ingredient_id in ingredient_ids {
        if fresh_ranges
            .iter()
            .any(|(a, b)| ingredient_id >= *a && ingredient_id <= *b)
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
        .map(|(a, b)| b - a + 1)
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
