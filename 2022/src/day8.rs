use std::collections::HashSet;

fn parse(input: Vec<String>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    for line in input {
        let mut tmp = Vec::new();
        for c in line.chars() {
            tmp.push(c as i32);
        }
        res.push(tmp);
    }
    res
}

pub fn run(input: Vec<String>) -> String {
    let width = input[0].len();
    let height = input.len();
    let parsed = parse(input);
    let mut visible_tree_heights = HashSet::new();

    for x in 0..width {
        for y in 0..height {
            let h = parsed[y][x];
            let mut visible = true;
            for xi in 0..x {
                if parsed[y][xi] >= h {
                    visible = false;
                }
            }
            if visible {
                visible_tree_heights.insert((x, y));
            }
            visible = true;
            for xi in x + 1..width {
                if parsed[y][xi] >= h {
                    visible = false;
                }
            }
            if visible {
                visible_tree_heights.insert((x, y));
            }
            visible = true;
            for yi in 0..y {
                if parsed[yi][x] >= h {
                    visible = false;
                }
            }
            if visible {
                visible_tree_heights.insert((x, y));
            }
            visible = true;
            for yi in y + 1..height {
                if parsed[yi][x] >= h {
                    visible = false;
                }
            }
            if visible {
                visible_tree_heights.insert((x, y));
            }
        }
    }

    visible_tree_heights.len().to_string()
}

#[test]
fn test() {
    let answer = "21".to_string();
    let input = vec![
        "30373".to_string(),
        "25512".to_string(),
        "65332".to_string(),
        "33549".to_string(),
        "35390".to_string(),
    ];
    assert_eq!(answer, run(input));
}
