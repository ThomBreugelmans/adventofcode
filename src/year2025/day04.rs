use macros::solution;

#[derive(PartialEq, Copy, Clone)]
enum CellType {
    PaperRoll,
    Empty,
}

enum Direction {
    UP = 0,
    DOWN = 1,
    LEFT = 2,
    RIGHT = 3,
    TOPLEFT = 4,
    TOPRIGHT = 5,
    BOTTOMLEFT = 6,
    BOTTOMRIGHT = 7,
}

const MAX: (usize, usize) = (150, 150);

fn get_neighbours(x: usize, y: usize) -> [Option<(usize, usize)>; 8] {
    let mut n = [None; 8];
    if x > 0 {
        n[Direction::LEFT as usize] = Some((x - 1, y));
    }
    if y > 0 {
        n[Direction::UP as usize] = Some((x, y - 1));
    }
    if x < MAX.0 - 1 {
        n[Direction::RIGHT as usize] = Some((x + 1, y));
    }
    if y < MAX.1 - 1 {
        n[Direction::DOWN as usize] = Some((x, y + 1));
    }
    if n[Direction::UP as usize].is_some() && n[Direction::LEFT as usize].is_some() {
        n[Direction::TOPLEFT as usize] = Some((x - 1, y - 1));
    }
    if n[Direction::UP as usize].is_some() && n[Direction::RIGHT as usize].is_some() {
        n[Direction::TOPRIGHT as usize] = Some((x + 1, y - 1));
    }
    if n[Direction::DOWN as usize].is_some() && n[Direction::LEFT as usize].is_some() {
        n[Direction::BOTTOMLEFT as usize] = Some((x - 1, y + 1));
    }
    if n[Direction::DOWN as usize].is_some() && n[Direction::RIGHT as usize].is_some() {
        n[Direction::BOTTOMRIGHT as usize] = Some((x + 1, y + 1));
    }
    n
}

fn parse(input: &str) -> ([[CellType; MAX.0]; MAX.1], (usize, usize)) {
    let mut grid = [[CellType::Empty; MAX.0]; MAX.1];
    let mut boundary = [0, 0];
    for (y, r) in input.trim().split('\n').enumerate() {
        boundary[1] = boundary[1].max(y);
        for (x, c) in r.chars().enumerate() {
            boundary[0] = boundary[0].max(x);
            grid[y][x] = match c {
                '@' => CellType::PaperRoll,
                _ => CellType::Empty,
            };
        }
    }
    (grid, (boundary[0] + 1, boundary[1] + 1))
}

fn get_movable_paperrolls(
    grid: &[[CellType; MAX.0]; MAX.1],
    boundary: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut movable = vec![];
    for y in 0..boundary.1 {
        let row = &grid[y];
        for x in 0..boundary.0 {
            let cell = row[x];
            if cell == CellType::Empty {
                continue;
            }
            let neigh = get_neighbours(x, y);
            (neigh
                .iter()
                .filter_map(|x| {
                    if let Some(n) = x {
                        if grid[n.1][n.0] == CellType::PaperRoll {
                            Some(())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .count()
                < 4)
            .then(|| movable.push((x, y)));
        }
    }
    movable
}

#[solution(year = 2025, day = 4, part = 1)]
fn part1(input: &str) -> String {
    let (parsed, boundary) = parse(input);
    let sum = get_movable_paperrolls(&parsed, boundary).len();
    sum.to_string()
}

#[solution(year = 2025, day = 4, part = 2)]
fn part2(input: &str) -> String {
    let (mut parsed, boundary) = parse(input);
    let mut sum = 0;
    loop {
        let movable = get_movable_paperrolls(&parsed, boundary);
        if movable.is_empty() {
            break;
        }
        sum += movable.len();
        for m in movable {
            parsed[m.1][m.0] = CellType::Empty;
        }
    }
    sum.to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

#[test]
fn test_part1() {
    let answer = "13";
    assert_eq!(answer, part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "43";
    assert_eq!(answer, part2(TEST_INPUT));
}
