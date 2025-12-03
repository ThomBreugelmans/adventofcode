use macros::solution;
use std::cmp::Ordering;

use nom::{
    character::complete::{alphanumeric1, space1},
    sequence::separated_pair,
};

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum BidType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

const POSSIBLE_CARDS: &str = "23456789TJQKA";
const POSSIBLE_CARDS_JOKER: &str = "J23456789TQKA";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Bid {
    hand: [char; 5],
    bid: u64,
    joker: bool,
}

impl Bid {
    fn get_type(&self) -> BidType {
        let mut count = [0; 13];
        for c in self.hand {
            let ind = (if self.joker {
                POSSIBLE_CARDS_JOKER
            } else {
                POSSIBLE_CARDS
            })
            .find(c)
            .unwrap();
            count[ind] += 1;
        }
        let mut cards = count
            .into_iter()
            .enumerate()
            .filter(|c| c.1 != 0)
            .collect::<Vec<_>>();
        cards.sort_by_key(|k| k.1);
        cards.reverse();

        if self.joker && cards.len() > 1 {
            if let Some(j_index) = cards.iter().position(|c| c.0 == 0) {
                let card = cards.remove(j_index);
                cards[0].1 += card.1;
            }
        }

        if cards[0].1 == 5 {
            BidType::FiveOfAKind
        } else if cards[0].1 == 4 {
            BidType::FourOfAKind
        } else if cards[0].1 == 3 && cards[1].1 == 2 {
            BidType::FullHouse
        } else if cards[0].1 == 3 {
            BidType::ThreeOfAKind
        } else if cards[0].1 == 2 && cards[1].1 == 2 {
            BidType::TwoPair
        } else if cards[0].1 == 2 {
            BidType::OnePair
        } else {
            BidType::HighCard
        }
    }
}

impl Ord for Bid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (self_type, other_type) = (self.get_type(), other.get_type());
        if self_type != other_type {
            self_type.cmp(&other_type)
        } else {
            for i in 0..5 {
                if self.hand[i] != other.hand[i] {
                    return cmp_card_a_to_b(self.hand[i], other.hand[i], self.joker);
                }
            }
            Ordering::Equal
        }
    }
}

impl PartialOrd for Bid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn cmp_card_a_to_b(a: char, b: char, joker: bool) -> std::cmp::Ordering {
    if a == b {
        Ordering::Equal
    } else {
        let (a_index, b_index) = if joker {
            (
                POSSIBLE_CARDS_JOKER.find(a).unwrap(),
                POSSIBLE_CARDS_JOKER.find(b).unwrap(),
            )
        } else {
            (
                POSSIBLE_CARDS.find(a).unwrap(),
                POSSIBLE_CARDS.find(b).unwrap(),
            )
        };
        a_index.cmp(&b_index)
    }
}

fn parse_bid(input: &str, joker: bool) -> Bid {
    if let Ok((_, res)) = separated_pair(
        alphanumeric1::<&str, nom::error::Error<&str>>,
        space1,
        alphanumeric1,
    )(input)
    {
        let mut b = Bid {
            hand: ['2'; 5],
            bid: res.1.parse::<u64>().unwrap(),
            joker,
        };
        for (i, c) in res.0.chars().enumerate() {
            b.hand[i] = c;
        }
        b
    } else {
        panic!("could not parse hand");
    }
}

fn parse(input: &str, joker: bool) -> Vec<Bid> {
    input
        .lines()
        .map(|l| parse_bid(l, joker))
        .collect::<Vec<_>>()
}

#[solution(year = 2023, day = 7, part = 1)]
fn run_part1(input: &str) -> String {
    let mut bids = parse(input, false);
    bids.sort();
    bids.into_iter()
        .enumerate()
        .map(|(i, b)| (i as u64 + 1) * b.bid)
        .sum::<u64>()
        .to_string()
}

#[solution(year = 2023, day = 7, part = 2)]
fn run_part2(input: &str) -> String {
    let mut bids = parse(input, true);
    bids.sort();
    bids.into_iter()
        .enumerate()
        .map(|(i, b)| (i as u64 + 1) * b.bid)
        .sum::<u64>()
        .to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[test]
fn test_part1() {
    let answer = "6440";
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "5905";
    assert_eq!(answer, run_part2(TEST_INPUT));
}
