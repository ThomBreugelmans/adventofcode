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

fn reflects(mirror: &Vec<Vec<char>>) -> Option<Fold> {
    (reflectivity(mirror)
        .into_iter()
        .position(|x| x == 0)
        .map(|r| Fold::Horizontal(r + 1)))
    .or(reflectivity(&transpose(mirror.clone()))
        .into_iter()
        .position(|x| x == 0)
        .map(|c| Fold::Vertical(c + 1)))
}

fn has_smudge(mirror: &Vec<Vec<char>>) -> Option<Fold> {
    (reflectivity(mirror)
        .into_iter()
        .position(|x| x == 1)
        .map(|r| Fold::Horizontal(r + 1)))
    .or(reflectivity(&transpose(mirror.clone()))
        .into_iter()
        .position(|x| x == 1)
        .map(|c| Fold::Vertical(c + 1)))
}

pub fn run(input: &str) -> String {
    format!("Part 1: {}\nPart 2: {}", run_part1(input), run_part2(input))
}

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

fn run_part2(input: &str) -> String {
    let mirrors = parse(input);
    mirrors
        .into_iter()
        .map(|m| match has_smudge(&m) {
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
