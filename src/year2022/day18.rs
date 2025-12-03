use macros::solution;
use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Type {
    Air = 0,
    Lava = 1,
    Water = 2,
}

fn parse(input: &str) -> [Type; 25 * 25 * 25] {
    let mut environment = [Type::Air; 25 * 25 * 25];
    for line in input.lines() {
        let xyz: Vec<_> = line
            .trim()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap() + 1)
            .collect();
        environment[xyz[0] * 25 * 25 + xyz[1] * 25 + xyz[2]] = Type::Lava;
    }
    environment
}

fn flood_fill(area: &mut [Type; 25 * 25 * 25]) {
    let mut frontier = VecDeque::new();
    frontier.push_back(0);
    while let Some(f) = frontier.pop_front() {
        let x = f / (25 * 25);
        let y = f % (25 * 25) / 25;
        let z = f % (25 * 25) % 25;
        if x < 0 || y < 0 || z < 0 || x >= 25 || y >= 25 || z >= 25 || area[f as usize] != Type::Air
        {
            continue;
        }
        area[f as usize] = Type::Water;
        let mut pos = [x, y, z];
        for i in 0..3 {
            for j in [-1, 1] {
                pos[i] += j;
                let p = pos[0] * 25 * 25 + pos[1] * 25 + pos[2];
                frontier.push_back(p);
                pos[i] -= j;
            }
        }
    }
}

#[solution(year = 2022, day = 18, part = 1)]
fn run_part1(input: &str) -> String {
    let cubes = parse(input);

    let mut neigh_count = HashMap::with_capacity(cubes.len());

    for (i, c) in cubes.iter().enumerate() {
        if *c != Type::Lava {
            continue;
        }
        neigh_count.insert(i, 0);
    }

    for (i, cube) in cubes.into_iter().enumerate() {
        if cube != Type::Lava {
            continue;
        }
        let x: i32 = (i / (25 * 25)) as i32;
        let y: i32 = ((i % (25 * 25)) / 25) as i32;
        let z: i32 = ((i % (25 * 25)) % 25) as i32;
        let mut neigh = [x, y, z];
        for j in [0, 1, 2] {
            for n in [-1, 1] {
                neigh[j] += n;
                if neigh[j] < 0 {
                    neigh[i] -= n;
                    continue;
                }
                if let Some(count) =
                    neigh_count.get_mut(&((neigh[0] * 25 * 25 + neigh[1] * 25 + neigh[2]) as usize))
                {
                    *count += 1;
                }
                neigh[j] -= n;
            }
        }
    }

    neigh_count.values().map(|v| 6 - v).sum::<u32>().to_string()
}

#[solution(year = 2022, day = 18, part = 2)]
fn run_part2(input: &str) -> String {
    let mut area = parse(input);
    flood_fill(&mut area);
    let mut count = 0;
    for i in area.iter().enumerate().filter_map(|(p, c)| {
        if *c != Type::Water {
            Some(p as i32)
        } else {
            None
        }
    }) {
        let x = i / (25 * 25);
        let y = i % (25 * 25) / 25;
        let z = i % (25 * 25) % 25;
        let mut pos = [x, y, z];
        for j in 0..3 {
            for k in [-1, 1] {
                if pos[j] + k < 0 || pos[j] + k >= 25 {
                    continue;
                }
                pos[j] += k;
                if area[(pos[0] * 25 * 25 + pos[1] * 25 + pos[2]) as usize] == Type::Water {
                    count += 1;
                }
                pos[j] -= k;
            }
        }
    }
    count.to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

#[test]
fn test_part1() {
    let answer = "64".to_string();
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "58".to_string();
    assert_eq!(answer, run_part2(TEST_INPUT));
}
