use pathfinding::prelude::dijkstra;
use std::cmp::min;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Node {
    valve: String,
    flow: u32,
    tunnels: Vec<String>,
}

impl Node {
    fn new(valve: String, flow: u32, tunnels: Vec<String>) -> Self {
        Self {
            valve,
            flow,
            tunnels,
        }
    }
}

fn parse(input: &Vec<String>) -> (String, HashMap<String, (u32, Vec<String>)>) {
    let mut valves = HashMap::new();
    let mut start = None;
    for line in input {
        let mut v = line.split([' ', ',', ';', '=']);
        let valve = v.nth(1).unwrap().to_string();
        if start.is_none() {
            start = Some(valve.clone());
        }
        let flow_rate = v.nth(3).unwrap().parse::<u32>().unwrap();

        let tunnels = v
            .skip(5)
            .filter_map(|s| {
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_string())
                }
            })
            .collect::<Vec<String>>();
        valves.insert(valve, (flow_rate, tunnels));
    }
    (start.unwrap(), valves)
}

fn find_optimal(
    start: &String,
    nodes: &HashMap<String, (String, u32, Vec<(String, u32)>)>,
    cur: (String, u32, Vec<(String, u32)>),
    visited: &mut HashMap<String, u32>,
    score: u32,
    minute: u32,
) -> (u32, String) {
    if minute == 30 || cur.2.len() == visited.len() {
        return (
            score + (visited.values().sum::<u32>() * (30 - minute)),
            cur.0,
        );
    }
    let mut m = u32::MIN;
    let mut p = "".to_string();

    for (valve, dist) in cur.2.iter() {
        if valve == start || visited.contains_key(valve) {
            continue;
        }

        let mut s = score + visited.values().sum::<u32>() * min(dist + 1, 30 - minute);
        let mut path_ = "".to_string();
        if (minute + dist + 1) <= 30 {
            visited.insert(valve.clone(), nodes.get(valve).unwrap().1);

            (s, path_) = find_optimal(
                start,
                nodes,
                nodes.get(valve).unwrap().clone(),
                visited,
                s,
                minute + dist + 1,
            );
            visited.remove(valve);
        }
        if s > m {
            m = s;
            p = path_;
        }
    }
    if p.is_empty() && m == 2048 {
        dbg!(minute, &visited, &cur);
    }

    let opt_path = format!("{} -> {}", cur.0, p);

    (m, opt_path)
}

pub fn run(input: Vec<String>) -> String {
    format!("{}\n{}", run_part1(&input), run_part2(&input))
}

fn run_part1(input: &Vec<String>) -> String {
    let (start, parsed) = parse(input);
    let nodes_of_value = parsed
        .iter()
        .filter_map(|(k, (flow, _))| {
            if *flow == 0 && *k != start {
                None
            } else {
                Some(k.clone())
            }
        })
        .collect::<Vec<String>>();

    // perform dijkstra
    let x = {
        let nodes = parsed
            .iter()
            .map(|(k, (f, s))| (k.clone(), Node::new(k.clone(), *f, s.clone())))
            .collect::<HashMap<String, Node>>();

        let mut res = HashMap::new();
        for node in nodes_of_value.iter() {
            let start = nodes.get(node).unwrap();
            let mut paths = Vec::<(String, u32)>::new();
            for g in nodes_of_value.iter() {
                if g == node {
                    continue;
                }
                let goal = nodes.get(g).unwrap();
                let (_, path_len): (Vec<Node>, u32) = dijkstra(
                    start,
                    |p| {
                        p.tunnels
                            .iter()
                            .map(|v| (nodes.get(v).unwrap().clone(), 1u32))
                            .collect::<Vec<(Node, u32)>>()
                    },
                    |n| n == goal,
                )
                .unwrap();
                paths.push((g.to_string(), path_len));
            }
            res.insert(node.clone(), (node.clone(), start.flow, paths));
        }

        res
    };

    let mut visited = HashMap::new();
    let (r, opt_path) = find_optimal(
        &start,
        &x,
        x.get(&start).unwrap().clone(),
        &mut visited,
        0,
        0,
    );

    format!("{} Path: {}", r, opt_path)
}

fn run_part2(input: &Vec<String>) -> String {
    let (start, parsed) = parse(input);
    "".to_string()
}

#[test]
fn test_part1() {
    let answer = "1651 Path: AA -> DD -> BB -> JJ -> HH -> EE -> CC".to_string();
    let input: Vec<String> = vec![
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".to_string(),
        "Valve BB has flow rate=13; tunnels lead to valves CC, AA".to_string(),
        "Valve CC has flow rate=2; tunnels lead to valves DD, BB".to_string(),
        "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE".to_string(),
        "Valve EE has flow rate=3; tunnels lead to valves FF, DD".to_string(),
        "Valve FF has flow rate=0; tunnels lead to valves EE, GG".to_string(),
        "Valve GG has flow rate=0; tunnels lead to valves FF, HH".to_string(),
        "Valve HH has flow rate=22; tunnel leads to valve GG".to_string(),
        "Valve II has flow rate=0; tunnels lead to valves AA, JJ".to_string(),
        "Valve JJ has flow rate=21; tunnel leads to valve II".to_string(),
    ];
    assert_eq!(answer, run_part1(&input));
}

#[test]
fn test_part2() {
    let answer = "".to_string();
    let input: Vec<String> = vec![
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".to_string(),
        "Valve BB has flow rate=13; tunnels lead to valves CC, AA".to_string(),
        "Valve CC has flow rate=2; tunnels lead to valves DD, BB".to_string(),
        "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE".to_string(),
        "Valve EE has flow rate=3; tunnels lead to valves FF, DD".to_string(),
        "Valve FF has flow rate=0; tunnels lead to valves EE, GG".to_string(),
        "Valve GG has flow rate=0; tunnels lead to valves FF, HH".to_string(),
        "Valve HH has flow rate=22; tunnel leads to valve GG".to_string(),
        "Valve II has flow rate=0; tunnels lead to valves AA, JJ".to_string(),
        "Valve JJ has flow rate=21; tunnel leads to valve II".to_string(),
    ];
    assert_eq!(answer, run_part2(&input));
}
