type Ores = [u32; 4];
type BuiltRobots = [u32; 4];
const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

struct Blueprint {
    max_ore: u32,
    max_clay: u32,
    max_obsidian: u32,
    costs: [Ores; 4],
}

impl Blueprint {
    fn from(line: &str) -> Self {
        let x: Vec<_> = line
            .split(' ')
            .filter_map(|v| v.parse::<u32>().ok())
            .collect();
        Blueprint {
            max_ore: x[0].max(x[1]).max(x[2].max(x[4])),
            max_clay: x[3],
            max_obsidian: x[5],
            costs: [
                [x[0], 0, 0, 0],
                [x[1], 0, 0, 0],
                [x[2], x[3], 0, 0],
                [x[4], 0, x[5], 0],
            ],
        }
    }
}

fn parse(input: &str) -> Vec<Blueprint> {
    let mut blueprints = Vec::new();
    for line in input.lines() {
        let bp = Blueprint::from(line);
        blueprints.push(bp);
    }
    blueprints
}

fn can_build(blueprint: &Blueprint, ores: Ores, bot_to_build: usize) -> bool {
    for i in 0..3 {
        if blueprint.costs[bot_to_build][i] > ores[i] {
            return false;
        }
    }
    true
}

fn maximize(blueprint: &Blueprint, time: usize) -> u32 {
    let mut result = 0;
    dfs(blueprint, [1, 0, 0, 0], [0, 0, 0, 0], &mut result, time);
    result
}

fn dfs(
    blueprint: &Blueprint,
    built_robots: BuiltRobots,
    ores: Ores,
    cur_max: &mut u32,
    minute: usize,
) {
    *cur_max = (*cur_max).max(ores[GEODE] + built_robots[GEODE] * minute as u32);

    if heuristic(blueprint, built_robots, ores, *cur_max, minute) {
        if built_robots[OBSIDIAN] > 0 && minute > 1 {
            next(blueprint, cur_max, minute, built_robots, ores, GEODE);
        }
        if built_robots[OBSIDIAN] < blueprint.max_obsidian && built_robots[CLAY] > 0 && minute > 3 {
            next(blueprint, cur_max, minute, built_robots, ores, OBSIDIAN);
        }
        if built_robots[ORE] < blueprint.max_ore && minute > 3 {
            next(blueprint, cur_max, minute, built_robots, ores, ORE);
        }
        if built_robots[CLAY] < blueprint.max_clay && minute > 5 {
            next(blueprint, cur_max, minute, built_robots, ores, CLAY);
        }
    }
}

fn heuristic(
    blueprint: &Blueprint,
    mut built_robots: BuiltRobots,
    mut ores: Ores,
    cur_max: u32,
    time: usize,
) -> bool {
    for _ in 0..time {
        ores[ORE] = blueprint.max_ore;
        if can_build(blueprint, ores, GEODE) {
            for i in 1..4 {
                ores[i] -= blueprint.costs[GEODE][i];
                ores[i] += built_robots[i];
            }
            built_robots[GEODE] += 1;
        } else if can_build(blueprint, ores, OBSIDIAN) {
            for i in 1..4 {
                ores[i] -= blueprint.costs[OBSIDIAN][i];
                ores[i] += built_robots[i];
            }
            built_robots[OBSIDIAN] += 1;
        } else {
            for i in 1..4 {
                ores[i] += built_robots[i];
            }
        }
        built_robots[CLAY] += 1;
    }
    ores[GEODE] > cur_max
}

fn next(
    blueprint: &Blueprint,
    cur_max: &mut u32,
    time: usize,
    mut built_robots: BuiltRobots,
    mut ores: Ores,
    bot_to_build: usize,
) {
    for jump in 1..time {
        if can_build(blueprint, ores, bot_to_build) {
            for i in 0..ores.len() {
                ores[i] -= blueprint.costs[bot_to_build][i];
                ores[i] += built_robots[i];
            }
            built_robots[bot_to_build] += 1;
            dfs(blueprint, built_robots, ores, cur_max, time - jump);
            break;
        }
        for i in 0..ores.len() {
            ores[i] += built_robots[i];
        }
    }
}

pub fn run(input: &str) -> String {
    format!("Part 1: {}\nPart 2: {}", run_part1(input), run_part2(input))
}

fn run_part1(input: &str) -> String {
    let blueprints = parse(input);
    blueprints
        .iter()
        .enumerate()
        .map(|(i, bp)| maximize(bp, 24) * (i + 1) as u32)
        .sum::<u32>()
        .to_string()
}

fn run_part2(input: &str) -> String {
    let blueprints = parse(input);
    blueprints
        .iter()
        .take(3)
        .map(|bp| maximize(bp, 32))
        .product::<u32>()
        .to_string()
}

const TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

#[test]
fn test_part1() {
    let answer = "33".to_string();
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "3472".to_string();
    assert_eq!(answer, run_part2(TEST_INPUT));
}
