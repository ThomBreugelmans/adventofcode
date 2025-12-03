use macros::solution;
use std::collections::VecDeque;

fn parse(input: &str) -> (Vec<Vec<i32>>, Vec<(i32, i32)>, (i32, i32)) {
    let mut grid = Vec::new();
    let mut start = Vec::new();
    let mut finish = (0, 0);
    for (y, line) in input.lines().enumerate() {
        grid.push(Vec::new());
        for (x, c) in line.chars().enumerate() {
            grid[y].push(match c {
                'S' | 'a' => {
                    start.push((x as i32, y as i32));
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

#[solution(year = 2022, day = 12, part = 2)]
pub fn run(input: &str) -> String {
    let (grid, starts, finish) = parse(input);
    let mut weights = Vec::new();
    for y in 0..grid.len() {
        weights.push(Vec::new());
        for _ in 0..grid[0].len() {
            weights[y].push(i32::MAX);
        }
    }

    let mut scores = Vec::new();

    for start in starts {
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

        scores.push(weights[finish.1 as usize][finish.0 as usize] - 1);
    }
    scores.iter().min().unwrap().to_string()
}

#[test]
fn test() {
    let answer = "29".to_string();
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    assert_eq!(answer, run(input));
}
