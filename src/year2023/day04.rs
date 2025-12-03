use macros::solution;
use std::collections::VecDeque;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0, space1};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};

#[derive(Debug, PartialEq, Eq, Clone)]
struct ScratchCard {
    id: usize,
    scratched_numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

fn parse(input: &str) -> Vec<ScratchCard> {
    let mut cards = Vec::new();
    for (id, line) in input.lines().enumerate() {
        let card = line.split(':').collect::<Vec<&str>>()[1];
        match separated_pair(
            preceded(
                space0::<&str, nom::error::Error<&str>>,
                separated_list1(space1, digit1),
            ),
            tag(" | "),
            preceded(space0, separated_list1(space1, digit1)),
        )(card)
        {
            Ok(("", (scratched, winning))) => cards.push(ScratchCard {
                id: id + 1,
                scratched_numbers: scratched
                    .into_iter()
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect(),
                winning_numbers: winning
                    .into_iter()
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect(),
            }),
            _ => (),
        }
    }
    cards
}

fn number_of_matches(s: &ScratchCard) -> usize {
    s.scratched_numbers
        .iter()
        .filter(|n| s.winning_numbers.contains(n))
        .count()
}

#[solution(year = 2023, day = 4, part = 1)]
fn run_part1(input: &str) -> String {
    let scratchcards = parse(input);
    scratchcards
        .into_iter()
        .filter_map(|s| {
            let x = number_of_matches(&s);
            if x > 0 {
                Some(2u32.pow(x.saturating_sub(1) as u32))
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}

#[solution(year = 2023, day = 4, part = 2)]
fn run_part2(input: &str) -> String {
    let scratchcards = parse(input);
    let mut scratched = 0u32;
    let mut frontier = VecDeque::new();
    scratchcards.iter().for_each(|s| frontier.push_back(s));
    while let Some(card) = frontier.pop_front() {
        scratched += 1;
        let x = number_of_matches(card);
        for new in (card.id)..(card.id + x) {
            frontier.push_back(&scratchcards[new]);
        }
    }
    scratched.to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

#[test]
fn test_part1() {
    let answer = "13";
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "30";
    assert_eq!(answer, run_part2(TEST_INPUT));
}
