use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit0, digit1};
use nom::sequence::separated_pair;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Lens<'a> {
    hash: usize,
    label: &'a str,
    focal: Option<usize>,
}

type HashMap<'a> = [Vec<Lens<'a>>; 256];

fn new_hashmap() -> HashMap<'static> {
    std::array::from_fn(|_| vec![])
}

fn remove(hm: &mut HashMap, lens: Lens) {
    hm[lens.hash]
        .iter()
        .position(|x| x.label == lens.label)
        .map(|i| hm[lens.hash].remove(i));
}

fn insert<'a>(hm: &mut HashMap<'a>, lens: Lens<'a>) {
    let hash = lens.hash;
    let pos = hm[hash].iter().position(|x| x.label == lens.label);
    match pos {
        Some(p) => {
            hm[hash][p].focal = lens.focal;
        }
        None => {
            hm[hash].push(lens);
        }
    }
}

fn compute_hash(label: &str) -> usize {
    label
        .as_bytes()
        .iter()
        .fold(0usize, |a, b| ((a + (*b) as usize) * 17) % 256)
}

fn parse(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

pub fn run(input: &str) -> String {
    format!("Part 1: {}\nPart 2: {}", run_part1(input), run_part2(input))
}

fn run_part1(input: &str) -> String {
    let sequences: Vec<&str> = parse(input);
    sequences
        .into_iter()
        .map(compute_hash)
        .sum::<usize>()
        .to_string()
}

fn run_part2(input: &str) -> String {
    let sequences = parse(input);
    let lenses: Vec<Lens> = sequences
        .into_iter()
        .map(|sequence| {
            alt((
                separated_pair(
                    alphanumeric1::<&str, nom::error::Error<&str>>,
                    tag("="),
                    digit1,
                ),
                separated_pair(alphanumeric1, tag("-"), digit0),
            ))(sequence)
            .map(|(_, p)| Lens {
                hash: compute_hash(p.0),
                label: p.0,
                focal: p.1.parse::<usize>().ok(),
            })
            .expect("was unable to parse")
        })
        .collect();

    let mut hashmap = new_hashmap();
    for lens in lenses.into_iter() {
        if lens.focal.is_some() {
            insert(&mut hashmap, lens);
        } else {
            remove(&mut hashmap, lens);
        }
    }

    hashmap
        .iter()
        .enumerate()
        .flat_map(|(bi, bx)| {
            bx.iter().enumerate().map(move |(li, lens)| {
                (bi + 1) * (li + 1) * lens.focal.expect("Lens in box should not be a None")
            })
        })
        .sum::<usize>()
        .to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

#[test]
fn test_part1() {
    let answer = "1320";
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "145";
    assert_eq!(answer, run_part2(TEST_INPUT));
}
