use std::collections::{HashMap, VecDeque};

fn parse(input: Vec<String>) -> (Vec<Vec<i32>>, (i32, i32), (i32, i32)) {
    let mut grid = Vec::new();
    let mut start = (0, 0);
    let mut finish = (0, 0);
    for (y, line) in input.iter().enumerate() {
        grid.push(Vec::new());
        for (x, c) in line.chars().enumerate() {
            grid[y].push(match c {
                'S' => {
                    start = (x as i32, y as i32);
                    0
                }
                'E' => {
                    finish = (x as i32, y as i32);
                    25
                }
                d => ((d as u8) - b'a') as i32,
            });
        }
    }
    (grid, start, finish)
}

pub fn run(input: Vec<String>) -> String {
    let (grid, start, finish) = parse(input);
    let mut weights = Vec::new();
    for y in 0..grid.len() {
        weights.push(Vec::new());
        for x in 0..grid[0].len() {
            weights[y].push(i32::MAX);
        }
    }

    let mut q = VecDeque::new();
    q.push_back((start, 0));

    while let Some(((x, y), mut weight)) = q.pop_front() {
        if weights[y as usize][x as usize] <= weight + 1 {
            continue;
        }
        weight += 1;
        weights[y as usize][x as usize] = weight;
        let neighs = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        for (nx, ny) in neighs {
            if nx < 0
                || nx >= grid[0].len() as i32
                || ny < 0
                || ny >= grid.len() as i32
                || grid[y as usize][x as usize] < grid[ny as usize][nx as usize] - 1
            {
                continue;
            }
            q.push_back(((nx, ny), weight));
        }
    }

    (weights[finish.1 as usize][finish.0 as usize] - 1).to_string()
}

#[test]
fn test() {
    let answer = "31".to_string();
    let input = vec![
        "Sabqponm".to_string(),
        "abcryxxl".to_string(),
        "accszExk".to_string(),
        "acctuvwj".to_string(),
        "abdefghi".to_string(),
    ];
    assert_eq!(answer, run(input));
}
