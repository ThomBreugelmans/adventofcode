use std::time::Instant;

use glam::i64::I64Vec3;
use itertools::Itertools;
use lib::graph::Graph;
use macros::solution;
use nom::{
    self, IResult, Parser,
    character::{self, complete},
    multi::separated_list1,
};

fn parse(input: &str) -> IResult<&str, Vec<I64Vec3>> {
    separated_list1(
        character::char('\n'),
        separated_list1(character::char(','), complete::i64)
            .map(|v| I64Vec3::new(v[0], v[1], v[2])),
    )
    .parse_complete(input.trim())
}

fn process<const N: usize>(input: Vec<I64Vec3>) -> usize {
    let mut now = Instant::now();
    let edges = input
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            input[i + 1..]
                .iter()
                .map(|b| (*a, *b, a.distance_squared(*b)))
        })
        .sorted_unstable_by_key(|(_, _, w)| *w)
        .take(N);
    println!("parsing took {} ms", now.elapsed().as_millis());

    now = Instant::now();
    let mut graph = Graph::new(input);
    for (from, to, weight) in edges {
        graph.add_edge(from, to, weight);
    }
    let clusters = graph.kruskal();
    println!("kruskal took {} ms", now.elapsed().as_millis());

    clusters
        .into_iter()
        .map(|c| c.len())
        .sorted_unstable()
        .rev()
        .take(3)
        .reduce(|a, b| a * b)
        .unwrap()
}

#[solution(year = 2025, day = 8, part = 1)]
fn part1(input: &str) -> String {
    let (_, parsed) = parse(input).unwrap();
    process::<1_000>(parsed).to_string()
}

#[solution(year = 2025, day = 8, part = 2)]
fn part2(input: &str) -> String {
    let (_, parsed) = parse(input).unwrap();
    let edges = parsed
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            parsed[i + 1..]
                .iter()
                .map(|b| (*a, *b, a.distance_squared(*b)))
        })
        .sorted_unstable_by_key(|(_, _, w)| *w);

    let mut graph = Graph::new(parsed);
    let mut last_edge = None;
    for (src, dst, w) in edges {
        graph.add_edge(src, dst, w);
        if graph.get_disjoint_graph_count() == 1 {
            last_edge = Some((src, dst));
            break;
        }
    }

    (last_edge.unwrap().0.x * last_edge.unwrap().1.x).to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

#[test]
fn test_part1() {
    let answer = 40;
    let (_, parsed) = parse(TEST_INPUT).unwrap();
    assert_eq!(answer, process::<10>(parsed));
}

#[test]
fn test_part2() {
    let answer = "25272";
    assert_eq!(answer, part2(TEST_INPUT));
}
