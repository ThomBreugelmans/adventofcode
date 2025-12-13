use std::u64;

use macros::solution;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes,
    character::{self, complete},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated},
};

fn button(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(
        bytes::tag(" ("),
        terminated(
            separated_list1(character::char(','), complete::usize),
            character::char(')'),
        ),
    )
    .parse(input)
}

fn buttons(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    nom::multi::fold_many1(button, Vec::new, |mut acc, v| {
        acc.push(v);
        acc
    })
    .parse(input)
}

fn joltages(input: &str) -> IResult<&str, [u8; 10]> {
    preceded(
        bytes::tag(" {"),
        terminated(
            separated_list1(character::char(','), complete::u8).map(|v| {
                let mut r = [0u8; 10];
                for (i, b) in v.into_iter().enumerate() {
                    r[i] = b;
                }
                r
            }),
            character::char('}'),
        ),
    )
    .parse(input)
}

fn lights(input: &str) -> IResult<&str, [bool; 10]> {
    preceded(
        character::char('['),
        terminated(
            many1(alt((
                complete::char('.').map(|_| false),
                complete::char('#').map(|_| true),
            )))
            .map(|v| {
                let mut r = [false; 10];
                for (i, b) in v.into_iter().enumerate() {
                    r[i] = b;
                }
                r
            }),
            character::char(']'),
        ),
    )
    .parse(input)
}

fn machine(input: &str) -> IResult<&str, ([bool; 10], Vec<Vec<usize>>, [u8; 10])> {
    lights
        .and(buttons)
        .and(joltages)
        .map(|((a, b), c)| (a, b, c))
        .parse(input)
}

fn parse(input: &str) -> Vec<([bool; 10], Vec<Vec<usize>>, [u8; 10])> {
    let (_, machines) = separated_list1(character::char('\n'), machine)
        .parse_complete(input.trim())
        .unwrap();
    machines
}

fn bf_search(target: [bool; 10], buttons: Vec<Vec<usize>>) -> u64 {
    let mut count = 0u64;
    let mut frontier = vec![[false; 10]];
    let mut new_frontier = Vec::new();
    'explore: while !frontier.is_empty() {
        // will not fail because of is_empty check
        let mut state = frontier.pop().unwrap();
        // sanity check
        if state == target {
            break;
        }
        for b in buttons.iter() {
            // modify state
            for &i in b {
                state[i] = !state[i];
            }
            // early check if target found
            if state == target {
                count += 1; // is after a button press, so needs to be updated
                break 'explore;
            }
            // add to new_frontier
            new_frontier.push(state.clone());
            // restore state
            for &i in b {
                state[i] = !state[i];
            }
        }
        if frontier.is_empty() {
            count += 1;
            // println!("Searching depth {count} for {:?}", target);
            frontier = new_frontier;
            new_frontier = Vec::new();
        }
    }
    count
}

#[solution(year = 2025, day = 10, part = 1)]
fn part1(input: &str) -> String {
    let parsed = parse(input);
    parsed
        .into_iter()
        .map(|(t, b, _)| bf_search(t, b))
        .sum::<u64>()
        .to_string()
}

// #[solution(year = 2025, day = 10, part = 2)]
// fn part2(input: &str) -> String {
//     let parsed = parse(input);
//     parsed
//         .into_iter()
//         .map(|(_, b, t)| bf_search2(t, b))
//         .sum::<u64>()
//         .to_string()
// }

#[allow(dead_code)]
const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

#[test]
fn test_part1() {
    let answer = "7";
    assert_eq!(answer, part1(TEST_INPUT));
}

// #[test]
// fn test_part2() {
//     let answer = "33";
//     assert_eq!(answer, part2(TEST_INPUT));
// }
