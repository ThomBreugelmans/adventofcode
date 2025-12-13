use glam::U64Vec2;
use macros::solution;
use nom::{
    IResult, Parser,
    character::{self, complete},
    multi::separated_list1,
    sequence::separated_pair,
};

fn parse(input: &str) -> IResult<&str, Vec<U64Vec2>> {
    separated_list1(
        character::char('\n'),
        separated_pair(complete::u64, character::char(','), complete::u64)
            .map(|p| U64Vec2::new(p.0, p.1)),
    )
    .parse_complete(input.trim())
}

#[solution(year = 2025, day = 9, part = 1)]
fn part1(input: &str) -> String {
    let (_, parsed) = parse(input).expect("Expected list of Vec2");
    let mut largest_area = 0u64;
    // let mut largest_pair = None;
    for i in 0..parsed.len() - 1 {
        let a = parsed[i];
        for j in i + 1..parsed.len() {
            let b = parsed[j];
            let area = (a.x.max(b.x) - a.x.min(b.x) + 1) * (a.y.max(b.y) - a.y.min(b.y) + 1);
            if area > largest_area {
                largest_area = area;
                // largest_pair = Some((a, b));
            }
        }
    }

    largest_area.to_string()
}

fn point_in_area(edges: &Vec<(U64Vec2, U64Vec2)>, point: U64Vec2) -> bool {
    let mut prev_dir: Option<(bool, bool)> = None;
    edges
        .iter()
        .filter(|(a, b)| {
            let min = a.min(*b);
            let max = a.max(*b);
            // only get vertical lines
            a.x == b.x
            // line has to contain point.y
            && (min.y..=max.y).contains(&point.y)
            // line has to be before the point
            && min.x <= point.x
        })
        .fold(false, |init, (a, b)| {
            match (a.x == point.x, a.y == point.y || b.y == point.y) {
                // should be the last statement given the filter min.x <= point.x
                // and edges is sorted
                (true, _) => true,
                (false, true) => {
                    let d = a.y > b.y;
                    if let Some((dir, state)) = prev_dir {
                        prev_dir = None;
                        if d == dir { !state } else { state }
                    } else {
                        prev_dir = Some((d, init));
                        true
                    }
                }
                (false, false) => !init,
            }
        })
}

#[solution(year = 2025, day = 9, part = 2)]
fn part2(input: &str) -> String {
    let (_, parsed) = parse(input).expect("Expected list of Vec2");
    let mut edges: Vec<(U64Vec2, U64Vec2)> = parsed[..parsed.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, p)| (*p, parsed[i + 1]))
        .collect();
    edges.push((parsed[parsed.len() - 1], parsed[0]));
    edges.sort_by_key(|(a, b)| a.min(*b).x);
    let mut largest_area = 0u64;
    for i in 0..parsed.len() - 1 {
        let a = parsed[i];
        for j in i + 1..parsed.len() {
            let b = parsed[j];
            let min_p = a.min(b);
            let max_p = a.max(b);
            let area = (max_p.x - min_p.x + 1) * (max_p.y - min_p.y + 1);
            if area > largest_area
                // test if no other points are within area
                && !parsed.iter().any(|p| (min_p.x+1..max_p.x).contains(&p.x) && (min_p.y+1..max_p.y).contains(&p.y))
                // look if all edges are red/green
                && (min_p.x..=max_p.x).flat_map(|x| [min_p.y,max_p.y].into_iter().map(move |y| U64Vec2::new(x,y))).all(|p| point_in_area(&edges, p))
                && (min_p.y+1..max_p.y).flat_map(|y| [min_p.x,max_p.x].into_iter().map(move |x| U64Vec2::new(x,y))).all(|p| point_in_area(&edges, p))
            {
                largest_area = area;
            }
        }
    }

    largest_area.to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

#[test]
fn test_part1() {
    let answer = "50";
    assert_eq!(answer, part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "24";
    assert_eq!(answer, part2(TEST_INPUT));
}
