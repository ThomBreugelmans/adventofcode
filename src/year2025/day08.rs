use glam::i64::I64Vec3;
use itertools::Itertools;
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
    let pairs: Vec<(i64, usize, usize)> = input
        .iter()
        .enumerate()
        .combinations(2)
        .map(|v| (v[0].1.distance_squared(*v[1].1), v[0].0, v[1].0))
        .filter(|x| x.1 != x.2)
        .sorted_by_key(|x| x.0)
        .take(N)
        .collect();

    let mut circuits: Vec<Vec<usize>> = Vec::with_capacity(input.len());
    let mut in_circuit = Vec::with_capacity(input.len());
    for (i, _) in input.iter().enumerate() {
        in_circuit.push(i);
        circuits.push(vec![i]);
    }
    for (_, src, dst) in pairs {
        if in_circuit[src] == in_circuit[dst] {
            // these are already in the same circuit
        } else {
            // not in the same circuit, add them to circuit of src
            let dst_circuit = in_circuit[dst];
            let src_circuit = in_circuit[src];
            circuits[src_circuit] = (&circuits[dst_circuit])
                .iter()
                .inspect(|c| in_circuit[**c] = src_circuit)
                .merge(&circuits[src_circuit])
                .map(|x| *x)
                .collect();
            circuits[dst_circuit].clear();
        }
    }

    circuits
        .into_iter()
        .map(|c| c.len())
        .sorted_unstable()
        .rev()
        .take(3)
        .reduce(|a, b| a * b)
        .expect("Should have had at least 1 circuit")
}

#[solution(year = 2025, day = 8, part = 1)]
fn part1(input: &str) -> String {
    let (_, parsed) = parse(input).unwrap();
    process::<1_000>(parsed).to_string()
}

#[solution(year = 2025, day = 8, part = 2)]
fn part2(input: &str) -> String {
    let (_, parsed) = parse(input).unwrap();
    let pairs: Vec<(i64, usize, usize)> = parsed
        .iter()
        .enumerate()
        .combinations(2)
        .map(|v| (v[0].1.distance_squared(*v[1].1), v[0].0, v[1].0))
        .filter(|x| x.1 != x.2)
        .sorted_by_key(|x| x.0)
        .collect();

    let mut circuit_count = parsed.len();
    let mut circuits: Vec<Vec<usize>> = Vec::with_capacity(parsed.len());
    let mut in_circuit = Vec::with_capacity(parsed.len());
    for (i, _) in parsed.iter().enumerate() {
        in_circuit.push(i);
        circuits.push(vec![i]);
    }
    let mut last_edge = None;
    for (_, src, dst) in pairs {
        if in_circuit[src] == in_circuit[dst] {
            // these are already in the same circuit
        } else {
            // not in the same circuit, add them to circuit of src
            let dst_circuit = in_circuit[dst];
            let src_circuit = in_circuit[src];
            circuits[src_circuit] = (&circuits[dst_circuit])
                .iter()
                .inspect(|c| in_circuit[**c] = src_circuit)
                .merge(&circuits[src_circuit])
                .map(|x| *x)
                .collect();
            circuits[dst_circuit].clear();
            circuit_count -= 1;
            last_edge = Some((src, dst));
            if circuit_count == 1 {
                break;
            }
        }
    }

    (parsed[last_edge.unwrap().0].x * parsed[last_edge.unwrap().1].x).to_string()
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
