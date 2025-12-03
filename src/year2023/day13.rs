use macros::solution;
use itertools::Itertools;

use crate::utils::grid::transpose;

fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|block| block.lines().map(|l| l.chars().collect()).collect())
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
enum Fold {
    Vertical(usize),
    Horizontal(usize),
}

/// finds the 'reflectivity of a mirror at each line
/// reflectivity indicates how many squares do not reflect over a given fold
fn reflectivity(mirror: &Vec<Vec<char>>) -> Vec<usize> {
    mirror
        .iter()
        .enumerate()
        .tuple_windows()
        .map(|(a, b)| {
            let a_side = &mirror[0..=a.0];
            let b_side = &mirror[b.0..mirror.len()];
            a_side
                .iter()
                .rev()
                .zip(b_side.iter())
                .map(|(l_a, l_b)| {
                    l_a.iter()
                        .zip(l_b.iter())
                        .filter(|(a_, b_)| a_ != b_)
                        .count()
                })
                .sum()
        })
        .collect()
}

fn find_fold_by_condition(mirror: &Vec<Vec<char>>, cond: impl Fn(usize) -> bool) -> Option<Fold> {
    (reflectivity(mirror)
        .into_iter()
        .position(&cond)
        .map(|r| Fold::Horizontal(r + 1)))
    .or(reflectivity(&transpose(mirror.clone()))
        .into_iter()
        .position(&cond)
        .map(|c| Fold::Vertical(c + 1)))
}

fn reflects(mirror: &Vec<Vec<char>>) -> Option<Fold> {
    find_fold_by_condition(mirror, |x| x == 0)
}

fn clean_smudge(mirror: &Vec<Vec<char>>) -> Option<Fold> {
    find_fold_by_condition(mirror, |x| x == 1)
}

#[solution(year=2023, day=13, part=1)]
fn run_part1(input: &str) -> String {
    let mirrors = parse(input);
    mirrors
        .into_iter()
        .map(|m| match reflects(&m) {
            Some(Fold::Vertical(c)) => c,
            Some(Fold::Horizontal(r)) => 100 * r,
            None => unreachable!("Each mirror should have a fold"),
        })
        .sum::<usize>()
        .to_string()
}

#[solution(year=2023, day=13, part=2)]
fn run_part2(input: &str) -> String {
    let mirrors = parse(input);
    mirrors
        .into_iter()
        .map(|m| match clean_smudge(&m) {
            Some(Fold::Vertical(c)) => c,
            Some(Fold::Horizontal(r)) => 100 * r,
            None => unreachable!("Each mirror should have a fold"),
        })
        .sum::<usize>()
        .to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

#[test]
fn test_part1() {
    let answer = "405";
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "400";
    assert_eq!(answer, run_part2(TEST_INPUT));
}
