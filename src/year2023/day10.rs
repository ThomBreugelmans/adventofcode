use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum GridCell {
    Empty,
    Occupied,
    Filled(u64),
}

type Grid = Vec<Vec<GridCell>>;

/// We get an input of pipes denoted by [|-FJL7] and need to compute the longest path to the starting position / 2
/// to do this we will enlarge the gotten grid by 3 to create actual tunnels and floodfill it
fn parse(input: &str) -> (Grid, (usize, usize)) {
    let mut grid_width = input.lines().next().unwrap().len() + 1;
    let grid_height = input.lines().count();
    let starting_index = input.find('S').expect("Could not find starting position");
    let start_x = starting_index % grid_width;
    let start_y = starting_index / grid_width;
    let mut connected = [false, false, false, false]; // [top, right, bottom, left]

    let bytes = input.as_bytes();
    if start_y > 0 && [b'|', b'F', b'7'].contains(&bytes[(start_y - 1) * grid_width + start_x]) {
        connected[0] = true;
    }
    if start_y < grid_height - 1
        && [b'|', b'L', b'J'].contains(&bytes[(start_y + 1) * grid_width + start_x])
    {
        connected[2] = true;
    }
    if start_x < grid_width - 1
        && [b'-', b'7', b'J'].contains(&bytes[start_y * grid_width + start_x + 1])
    {
        connected[1] = true;
    }
    if start_x > 0 && [b'-', b'L', b'F'].contains(&bytes[start_y * grid_width + start_x - 1]) {
        connected[3] = true;
    }

    grid_width = grid_width.saturating_sub(1);

    let mut grid = Vec::new();
    for _ in 0..grid_height * 3 {
        let mut row = Vec::new();
        row.reserve(grid_width);
        for _ in 0..grid_width * 3 {
            row.push(GridCell::Empty);
        }
        grid.push(row);
    }

    let mut x = 0;
    let mut y = 0;
    for c in input.chars() {
        let sub = match c {
            'S' => {
                let mut sub = [-1, -1, -1, -1, 0, -1, -1, -1, -1];
                sub.iter_mut().enumerate().for_each(|(o, s)| {
                    *s = match o {
                        1 if connected[0] => 0,
                        3 if connected[3] => 0,
                        5 if connected[1] => 0,
                        7 if connected[2] => 0,
                        _ => *s,
                    }
                });

                sub
            }
            'F' => [-1, -1, -1, -1, 0, 0, -1, 0, -1],
            'J' => [-1, 0, -1, 0, 0, -1, -1, -1, -1],
            'L' => [-1, 0, -1, -1, 0, 0, -1, -1, -1],
            '7' => [-1, -1, -1, 0, 0, -1, -1, 0, -1],
            '-' => [-1, -1, -1, 0, 0, 0, -1, -1, -1],
            '|' => [-1, 0, -1, -1, 0, -1, -1, 0, -1],
            '\n' => {
                x = 0;
                y += 1;
                continue;
            }
            _ => [0, 0, 0, 0, 0, 0, 0, 0, 0],
        };

        for (o, s) in sub.into_iter().enumerate() {
            grid[y * 3 + (o / 3)][x * 3 + (o % 3)] = if s == 0 {
                GridCell::Empty
            } else {
                GridCell::Occupied
            };
        }

        x += 1;
    }
    (grid, (start_x * 3 + 1, start_y * 3 + 1))
}

fn flood_fill(grid: &mut Grid, start: (usize, usize)) {
    let mut frontier = VecDeque::new();
    frontier.push_back((start, 0));

    while let Some(((x, y), d)) = frontier.pop_front() {
        if grid[y][x] != GridCell::Empty {
            continue;
        }
        grid[y][x] = GridCell::Filled(d);

        for n in [
            (x + 1, y),
            (x, y + 1),
            (x.saturating_sub(1), y),
            (x, y.saturating_sub(1)),
        ] {
            if n.0 < grid[y].len() && n.1 < grid.len() {
                frontier.push_back((n, d + 1));
            }
        }
    }
}

fn filled_to_occupied(grid: &mut Grid) {
    grid.iter_mut().for_each(|r| {
        r.iter_mut().for_each(|c| {
            *c = if let GridCell::Filled(_) = c {
                GridCell::Occupied
            } else {
                GridCell::Empty
            }
        })
    });
}

fn count(grid: &Grid) -> usize {
    let mut count = 0;
    for y in 0..grid.len() / 3 {
        'outer: for x in 0..grid[y].len() / 3 {
            for o in 0..9 {
                if grid[y * 3 + (o / 3)][x * 3 + (o % 3)] != GridCell::Empty {
                    continue 'outer;
                }
            }
            count += 1;
        }
    }
    count
}

pub fn run(input: &str) -> String {
    format!("Part 1: {}\nPart 2: {}", run_part1(input), run_part2(input))
}

fn run_part1(input: &str) -> String {
    let (mut grid, start) = parse(input);
    flood_fill(&mut grid, start);

    (grid
        .into_iter()
        .flatten()
        .map(|c| match c {
            GridCell::Filled(d) => d,
            _ => 0,
        })
        .max()
        .unwrap()
        / 3)
    .to_string()
}

fn run_part2(input: &str) -> String {
    let (mut grid, start) = parse(input);
    flood_fill(&mut grid, start);
    filled_to_occupied(&mut grid);
    flood_fill(&mut grid, (0, 0));

    count(&grid).to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
#[allow(dead_code)]
const TEST_INPUT2: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
#[allow(dead_code)]
const TEST_INPUT3: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
#[allow(dead_code)]
const TEST_INPUT4: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
#[allow(dead_code)]
const TEST_INPUT5: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
#[allow(dead_code)]
const TEST_INPUT6: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

#[test]
fn test_part1_1() {
    let answer = "4";
    assert_eq!(answer, run_part1(TEST_INPUT));
}
#[test]
fn test_part1_2() {
    let answer = "4";
    assert_eq!(answer, run_part1(TEST_INPUT2));
}
#[test]
fn test_part1_3() {
    let answer = "8";
    assert_eq!(answer, run_part1(TEST_INPUT3));
}

#[test]
fn test_part2_1() {
    let answer = "4";
    assert_eq!(answer, run_part2(TEST_INPUT4));
}

#[test]
fn test_part2_2() {
    let answer = "8";
    assert_eq!(answer, run_part2(TEST_INPUT5));
}

#[test]
fn test_part2_3() {
    let answer = "10";
    assert_eq!(answer, run_part2(TEST_INPUT6));
}
