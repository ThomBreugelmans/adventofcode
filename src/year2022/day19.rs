type Blueprint = [Robot; 4];
type Ores = [u32; 4];
type BuiltRobots = [u32; 4];
const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

enum Robot {
    Ore(u32),
    Clay(u32),
    Obsidian(u32, u32),
    Geode(u32, u32),
}

fn parse(input: &str) -> Vec<Blueprint> {
    let mut blueprints = Vec::new();
    for line in input.lines() {
        let x: Vec<_> = line
            .split(' ')
            .filter_map(|v| v.parse::<u32>().ok())
            .collect();
        let bp = [
            Robot::Ore(x[0]),
            Robot::Clay(x[1]),
            Robot::Obsidian(x[2], x[3]),
            Robot::Geode(x[4], x[5]),
        ];
        blueprints.push(bp);
    }
    blueprints
}

fn calc_return_of_investment(
    blueprint: &Blueprint,
    built_robots: BuiltRobots,
    ores: Ores,
    cur_max: u32,
    minute: usize,
) -> u32 {
}

fn heuristic(
    blueprint: &Blueprint,
    mut built_robots: BuiltRobots,
    mut ores: Ores,
    cur_max: u32,
    time: usize,
) -> bool {
    for _ in 0..time {
        let Robot::Geode(_, obs_cost) = blueprint[GEODE];
        let Robot::Obsidian(_, clay_cost) = blueprint[OBSIDIAN];
        if ores[OBSIDIAN] >= obs_cost {
            ores[OBSIDIAN] -= obs_cost;
            built_robots[GEODE] += 1;
        } else if ores[CLAY] >= clay_cost {
            ores[CLAY] -= clay_cost;
            built_robots[OBSIDIAN] += 1;
        }
        for i in 0..ores.len() {
            ores[i] += built_robots[i];
        }
        built_robots[CLAY] += 1;
    }
    ores[GEODE] > cur_max
}

pub fn run(input: &str) -> String {
    format!(
        "Part 1: {}\n Part 2: {}",
        run_part1(input),
        run_part2(input)
    )
}

fn run_part1(input: &str) -> String {
    let blueprints = parse(input);
    "".to_string()
}

fn run_part2(input: &str) -> String {
    "".to_string()
}

const TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

#[test]
fn test_part1() {
    let answer = "".to_string();
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "".to_string();
    assert_eq!(answer, run_part2(TEST_INPUT));
}
