use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: Vec<String>) -> Vec<Direction> {
    let mut actions = Vec::new();
    for line in input {
        match line.get(0..1) {
            Some("U") => {
                for _ in 0..line.get(2..).unwrap().parse::<i32>().unwrap() {
                    actions.push(Direction::Up);
                }
            }
            Some("D") => {
                for _ in 0..line.get(2..).unwrap().parse::<i32>().unwrap() {
                    actions.push(Direction::Down);
                }
            }
            Some("L") => {
                for _ in 0..line.get(2..).unwrap().parse::<i32>().unwrap() {
                    actions.push(Direction::Left);
                }
            }
            Some("R") => {
                for _ in 0..line.get(2..).unwrap().parse::<i32>().unwrap() {
                    actions.push(Direction::Right);
                }
            }
            _ => panic!(),
        };
    }

    actions
}

struct Node {
    x: i32,
    y: i32,
}

pub fn run(input: Vec<String>) -> String {
    let directions = parse(input);
    let mut head = Node { x: 0, y: 0 };
    let mut tail = Node { x: 0, y: 0 };
    let mut visited = HashSet::new();
    visited.insert((tail.x, tail.y));

    for d in directions {
        let new_head = match d {
            Direction::Up => Node {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Down => Node {
                x: head.x,
                y: head.y - 1,
            },
            Direction::Left => Node {
                x: head.x - 1,
                y: head.y,
            },
            Direction::Right => Node {
                x: head.x + 1,
                y: head.y,
            },
        };
        if tail.x.abs_diff(new_head.x) > 1 || tail.y.abs_diff(new_head.y) > 1 {
            tail = Node {
                x: head.x,
                y: head.y,
            };
        }
        visited.insert((tail.x, tail.y));
        head = new_head;
    }
    visited.len().to_string()
}

#[test]
fn test() {
    let answer = "13".to_string();
    let input = vec![
        "R 4".to_string(),
        "U 4".to_string(),
        "L 3".to_string(),
        "D 1".to_string(),
        "R 4".to_string(),
        "D 1".to_string(),
        "L 5".to_string(),
        "R 2".to_string(),
    ];

    assert_eq!(answer, run(input));
}
