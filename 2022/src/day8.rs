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
    let mut scenic_score = 0;

    for x in 0..width {
        for y in 0..height {
            let h = parsed[y][x];
            let mut new_score = 1;
            for xi in (0..=x).rev() {
                if (x != xi && parsed[y][xi] >= h) || xi == 0 {
                    new_score *= xi.abs_diff(x);
                    break;
                }
            }
            for xi in x..width {
                if (xi != x && parsed[y][xi] >= h) || xi == width - 1 {
                    new_score *= xi.abs_diff(x);
                    break;
                }
            }
            for yi in (0..=y).rev() {
                if (yi != y && parsed[yi][x] >= h) || yi == 0 {
                    new_score *= yi.abs_diff(y);
                    break;
                }
            }
            for yi in y..height {
                if (yi != y && parsed[yi][x] >= h) || yi == height - 1 {
                    new_score *= yi.abs_diff(y);
                    break;
                }
            }
            if new_score > scenic_score {
                scenic_score = new_score;
            }
        }
    }

    scenic_score.to_string()
}

#[test]
fn test() {
    let answer = "8".to_string();
    let input = vec![
        "30373".to_string(),
        "25512".to_string(),
        "65332".to_string(),
        "33549".to_string(),
        "35390".to_string(),
    ];
    assert_eq!(answer, run(input));
}
