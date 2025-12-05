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

fn is_movable(grid: &[[CellType; MAX.0]; MAX.1], x: usize, y: usize) -> bool {
    if grid[y][x] == CellType::Empty {
        return false;
    }
    let neigh = get_neighbours(x, y);
    neigh
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
        < 4
}

fn get_movable_paperrolls(
    grid: &[[CellType; MAX.0]; MAX.1],
    boundary: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut movable = vec![];
    for y in 0..boundary.1 {
        for x in 0..boundary.0 {
            is_movable(grid, x, y).then(|| movable.push((x, y)));
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

// fn grid2str(grid: &[[CellType; MAX.0]; MAX.1], boundary: (usize, usize)) -> String {
//     let mut string = String::new();
//     for (j, r) in grid.iter().enumerate() {
//         if j >= boundary.1 {
//             break;
//         }
//         let r_str = r
//             .iter()
//             .enumerate()
//             .filter(|(i, _)| *i < boundary.0)
//             .map(|(_, c)| match c {
//                 CellType::PaperRoll => '@',
//                 _ => '.',
//             })
//             .fold(String::from(""), |i, x| format!("{i}{x}"));
//         string = format!("{}\n{}", string, r_str);
//     }
//     string
// }

#[solution(year = 2025, day = 4, part = 2)]
fn part2(input: &str) -> String {
    // println!("{input}\n\n");
    let (mut parsed, boundary) = parse(input);
    let mut sum = 0;
    let mut candidates = get_movable_paperrolls(&parsed, boundary);
    loop {
        if candidates.is_empty() {
            break;
        }
        let movable = candidates.pop().unwrap();
        if parsed[movable.1][movable.0] == CellType::Empty {
            continue;
        }
        sum += 1;
        parsed[movable.1][movable.0] = CellType::Empty;
        // Only consider those surrounding the movable for the next
        for c in get_neighbours(movable.0, movable.1)
            .into_iter()
            .filter_map(|c| match c {
                Some((x, y)) => is_movable(&parsed, x, y).then_some((x, y)),
                None => None,
            })
        {
            candidates.push(c);
        }
    }
    // println!("{}", grid2str(&parsed, boundary));
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
