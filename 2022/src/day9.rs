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

#[derive(Clone, Debug)]
struct Node {
    x: i32,
    y: i32,
}

pub fn run(input: Vec<String>) -> String {
    let directions = parse(input);
    let mut rope: [Node; 10] = [
        Node { x: 0, y: 0 },
        Node { x: 0, y: 0 },
        Node { x: 0, y: 0 },
        Node { x: 0, y: 0 },
        Node { x: 0, y: 0 },
        Node { x: 0, y: 0 },
        Node { x: 0, y: 0 },
        Node { x: 0, y: 0 },
        Node { x: 0, y: 0 },
        Node { x: 0, y: 0 },
    ];
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for d in directions {
        let new_head = match d {
            Direction::Up => Node {
                x: rope[0].x,
                y: rope[0].y + 1,
            },
            Direction::Down => Node {
                x: rope[0].x,
                y: rope[0].y - 1,
            },
            Direction::Left => Node {
                x: rope[0].x - 1,
                y: rope[0].y,
            },
            Direction::Right => Node {
                x: rope[0].x + 1,
                y: rope[0].y,
            },
        };

        fn update_rope(rope: &mut [Node; 10], n: &Node, i: usize) {
            if i == 0 {
                update_rope(rope, n, i + 1);
                rope[i] = n.clone();
            } else {
                if i >= 10 {
                    return;
                }
                if rope[i].x.abs_diff(n.x) > 1 || rope[i].y.abs_diff(n.y) > 1 {
                    let x = Node {
                        x: rope[i].x + (n.x - rope[i].x).clamp(-1, 1),
                        y: rope[i].y + (n.y - rope[i].y).clamp(-1, 1),
                    };
                    update_rope(rope, &x, i + 1);
                    rope[i] = x;
                }
            }
        }

        update_rope(&mut rope, &new_head, 0);
        visited.insert((rope[9].x, rope[9].y));
    }
    visited.len().to_string()
}

#[test]
fn test() {
    let answer = "36".to_string();
    let input = vec![
        "R 5".to_string(),
        "U 8".to_string(),
        "L 8".to_string(),
        "D 3".to_string(),
        "R 17".to_string(),
        "D 10".to_string(),
        "L 25".to_string(),
        "U 20".to_string(),
    ];

    assert_eq!(answer, run(input));
}
