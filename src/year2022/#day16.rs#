use pathfinding::prelude::dijkstra;
use std::collections::{HashMap, HashSet};

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

fn parse(input: &Vec<String>) -> HashMap<String, (u32, Vec<String>)> {
    let mut valves = HashMap::new();

    for line in input {
        let mut v = line.split([' ', ',', ';', '=']);
        let valve = v.nth(1).unwrap().to_string();
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
    valves
}

fn get_reduced_net(
    parsed: HashMap<String, (u32, Vec<String>)>,
) -> HashMap<String, (String, u32, Vec<(String, u32)>)> {
    let nodes_of_value = parsed
        .iter()
        .filter_map(|(k, (flow, _))| {
            if *flow == 0 && *k != "AA" {
                None
            } else {
                Some(k.clone())
            }
        })
        .collect::<Vec<String>>();
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
    x
}

fn get_combinations_within_time(
    pos: (String, u32, Vec<(String, u32)>),
    unvisited: &mut HashSet<String>,
    nodes: &HashMap<String, (String, u32, Vec<(String, u32)>)>,
    time_left: i32,
) -> HashSet<Vec<String>> {
    let mut res = HashSet::new();

    if time_left <= 0 {
        res.insert(Vec::new());
    } else {
        for (next, dist) in pos.2 {
            if !unvisited.contains(&next) {
                continue;
            }
            unvisited.remove(&next);
            get_combinations_within_time(
                nodes.get(&next).unwrap().clone(),
                unvisited,
                nodes,
                time_left - dist as i32 - 1,
            )
            .into_iter()
            .for_each(|mut v| {
                let mut a = vec![pos.0.clone()];
                a.append(&mut v);
                res.insert(a);
            });
            unvisited.insert(next);
        }
        res.insert(vec![pos.0.clone()]);
    }

    res
}

fn get_score_of_path(
    path: Vec<String>,
    nodes: &HashMap<String, (String, u32, Vec<(String, u32)>)>,
    limit: u32,
    memoise: &mut HashMap<Vec<String>, u32>,
) -> u32 {
    let mut minute = 0;
    let mut score = 0;

    let mut iter = path.iter();
    let mut cur = nodes.get(iter.next().unwrap()).unwrap();
    let mut done = vec![cur.0.clone()];

    for n in iter {
        let (_, dist) = cur.2[cur.2.iter().position(|(x, _)| x == n).unwrap()];
        cur = nodes.get(n).unwrap();
        done.push(cur.0.clone());
        score += (limit - minute - dist - 1) * cur.1;
        memoise.insert(done.clone(), score);
        minute += dist + 1;
    }

    score
}

pub fn run(input: Vec<String>) -> String {
    let mut memoise = HashMap::new();
    format!(
        "{}\n{}",
        run_part1(&input, &mut memoise),
        run_part2(&input, &mut memoise)
    )
}

fn run_part1(input: &Vec<String>, memoise: &mut HashMap<Vec<String>, u32>) -> String {
    let start = "AA".to_string();
    let parsed = parse(input);

    // perform dijkstra
    let x = get_reduced_net(parsed);

    let mut unvisited = x.keys().cloned().collect::<HashSet<String>>();
    unvisited.insert("AA".to_string());
    let possible_combinations =
        get_combinations_within_time(x.get(&start).unwrap().clone(), &mut unvisited, &x, 30);

    //let (r, opt_path) = find_optimal(&x, x.get(&start).unwrap().clone(), &mut unvisited, 0, 0, 30);

    //   let opt_path_string = opt_path
    //       .into_iter()
    //       .reduce(|a, b| format!("{} -> {}", a, b))
    //       .unwrap();
    // format!("{} Path: {}", r, opt_path_string)
    possible_combinations
        .into_iter()
        .map(|path| get_score_of_path(path, &x, 30, memoise))
        .max()
        .unwrap()
        .to_string()
}

fn run_part2(input: &Vec<String>, memoise: &mut HashMap<Vec<String>, u32>) -> String {
    let parsed = parse(input);
    let x = get_reduced_net(parsed);

    let mut ccs = get_combinations_within_time(
        x.get(&"AA".to_string()).unwrap().clone(),
        &mut x.keys().cloned().collect(),
        &x,
        26,
    );
    ccs.remove(&vec!["AA".to_string()]);

    ccs.iter().for_each(|path| {
        get_score_of_path(path.clone(), &x, 26, memoise);
    });

    let mut max_val = u32::MIN;
    for h_path in ccs.iter() {
        let h_max = memoise.get(h_path).unwrap();
        let e_paths = ccs
            .iter()
            .filter(|p| p.iter().all(|e| !h_path.contains(e) || e == "AA"));

        for e_path in e_paths {
            let e_max = memoise.get(e_path).unwrap();
            if max_val < *h_max + *e_max {
                max_val = *h_max + *e_max;
            }
        }
    }

    max_val.to_string()
}

#[test]
fn test_part1() {
    let answer = "1651".to_string();
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
    assert_eq!(answer, run_part1(&input, &mut HashMap::new()));
}
/*
#[test]
fn test_part1_2() {
    let answer =
        "2640 Path: AA -> FA -> GA -> HA -> IA -> JA -> KA -> LA -> MA -> NA -> OA -> PA -> "
            .to_string();
    let input = vec![
        "Valve LA has flow rate=22; tunnels lead to valves KA, MA".to_string(),
        "Valve MA has flow rate=24; tunnels lead to valves LA, NA".to_string(),
        "Valve NA has flow rate=26; tunnels lead to valves MA, OA".to_string(),
        "Valve OA has flow rate=28; tunnels lead to valves NA, PA".to_string(),
        "Valve PA has flow rate=30; tunnels lead to valves OA".to_string(),
        "Valve AA has flow rate=0; tunnels lead to valves BA".to_string(),
        "Valve BA has flow rate=2; tunnels lead to valves AA, CA".to_string(),
        "Valve CA has flow rate=4; tunnels lead to valves BA, DA".to_string(),
        "Valve DA has flow rate=6; tunnels lead to valves CA, EA".to_string(),
        "Valve EA has flow rate=8; tunnels lead to valves DA, FA".to_string(),
        "Valve FA has flow rate=10; tunnels lead to valves EA, GA".to_string(),
        "Valve GA has flow rate=12; tunnels lead to valves FA, HA".to_string(),
        "Valve HA has flow rate=14; tunnels lead to valves GA, IA".to_string(),
        "Valve IA has flow rate=16; tunnels lead to valves HA, JA".to_string(),
        "Valve JA has flow rate=18; tunnels lead to valves IA, KA".to_string(),
        "Valve KA has flow rate=20; tunnels lead to valves JA, LA".to_string(),
    ];
    assert_eq!(answer, run_part1(&input))
}

#[test]
fn test_part1_3() {
    let answer = "13468 AA -> IA -> JA -> KA -> LA -> MA -> NA -> OA -> PA".to_string();
    let input = vec![
        "Valve AA has flow rate=0; tunnels lead to valves BA".to_string(),
        "Valve BA has flow rate=1; tunnels lead to valves AA, CA".to_string(),
        "Valve CA has flow rate=4; tunnels lead to valves BA, DA".to_string(),
        "Valve DA has flow rate=9; tunnels lead to valves CA, EA".to_string(),
        "Valve EA has flow rate=16; tunnels lead to valves DA, FA".to_string(),
        "Valve FA has flow rate=25; tunnels lead to valves EA, GA".to_string(),
        "Valve GA has flow rate=36; tunnels lead to valves FA, HA".to_string(),
        "Valve HA has flow rate=49; tunnels lead to valves GA, IA".to_string(),
        "Valve IA has flow rate=64; tunnels lead to valves HA, JA".to_string(),
        "Valve JA has flow rate=81; tunnels lead to valves IA, KA".to_string(),
        "Valve KA has flow rate=100; tunnels lead to valves JA, LA".to_string(),
        "Valve LA has flow rate=121; tunnels lead to valves KA, MA".to_string(),
        "Valve MA has flow rate=144; tunnels lead to valves LA, NA".to_string(),
        "Valve NA has flow rate=169; tunnels lead to valves MA, OA".to_string(),
        "Valve OA has flow rate=196; tunnels lead to valves NA, PA".to_string(),
        "Valve PA has flow rate=225; tunnels lead to valves OA".to_string(),
    ];
    assert_eq!(answer, run_part1(&input));
}
*/
#[test]
fn test_part2() {
    let answer = "1707".to_string();
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
    assert_eq!(answer, run_part2(&input, &mut HashMap::new()));
}
