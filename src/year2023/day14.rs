use macros::solution;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Object {
    RoundRock,
    Rock,
    Empty,
}

fn parse(input: &str) -> Vec<Vec<Object>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'O' => Object::RoundRock,
                    '#' => Object::Rock,
                    _ => Object::Empty,
                })
                .collect()
        })
        .collect()
}

fn rotate_clockwise(grid: Vec<Vec<Object>>) -> Vec<Vec<Object>> {
    let len = grid[0].len();
    let mut iters: Vec<_> = grid.into_iter().map(|x| x.into_iter()).rev().collect();
    (0..len)
        .map(|_| iters.iter_mut().map(|x| x.next().unwrap()).collect())
        .collect()
}

fn move_north(grid: Vec<Vec<Object>>) -> Vec<Vec<Object>> {
    let mut indices: Vec<usize> = grid[0].clone().into_iter().map(|_| 0usize).collect();
    let mut new_grid = grid.clone();

    for (r_i, r) in grid.into_iter().enumerate() {
        for (i, o) in r.into_iter().enumerate() {
            match o {
                Object::RoundRock => {
                    new_grid[r_i][i] = Object::Empty;
                    new_grid[indices[i]][i] = Object::RoundRock;
                    indices[i] += 1;
                }
                Object::Rock => indices[i] = r_i + 1,
                Object::Empty => (),
            }
        }
    }

    new_grid
}

#[solution(year=2023, day=14, part=1)]
fn run_part1(input: &str) -> String {
    let mut grid = parse(input);
    grid = move_north(grid);

    let grid_len = grid.len();
    grid.into_iter()
        .enumerate()
        .map(|(i, x)| (grid_len - i) * x.into_iter().filter(|y| *y == Object::RoundRock).count())
        .sum::<usize>()
        .to_string()
}

#[solution(year=2023, day=14, part=2)]
fn run_part2(input: &str) -> String {
    let mut grid = parse(input);
    let mut cache = HashMap::new();

    let mut progress = 0;
    while progress < 1_000_000_000 {
        for _ in 0..4 {
            grid = rotate_clockwise(move_north(grid))
        }

        let key = grid
            .iter()
            .enumerate()
            .flat_map(|(r_i, r)| {
                r.iter()
                    .enumerate()
                    .filter_map(|(c_i, c)| (c == &Object::RoundRock).then_some((c_i, r_i)))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        progress += 1;
        if let Some(cycle_start) = cache.get(&key) {
            let cycle_period = progress - cycle_start;
            progress = ((1_000_000_000 - cycle_start) / cycle_period) * cycle_period + cycle_start;
        } else {
            cache.insert(key, progress);
        }
    }

    let grid_len = grid.len();
    grid.into_iter()
        .enumerate()
        .map(|(i, x)| (grid_len - i) * x.into_iter().filter(|y| *y == Object::RoundRock).count())
        .sum::<usize>()
        .to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

#[test]
fn test_part1() {
    let answer = "136";
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "64";
    assert_eq!(answer, run_part2(TEST_INPUT));
}
