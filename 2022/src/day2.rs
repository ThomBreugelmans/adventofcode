use std::fs::read_to_string;

pub fn run() {
    // get input code:
    let input: Vec<String> = {
        let mut _input = Vec::new();
        for line in read_to_string("input/day2").unwrap().lines() {
            _input.push(line.to_string());
        }
        _input
    };

    let output = _run(input);
    println!("{}", output);
}

fn score(opponent: i8, you: i8) -> i32 {
    match you {
        0 => (((opponent - 1).rem_euclid(3)) + 1) as i32,
        1 => (opponent + 1 + 3) as i32,
        2 => (((opponent + 1).rem_euclid(3)) + 1 + 6) as i32,
        _ => unimplemented!(),
    }
}

fn _run(input: Vec<String>) -> i32 {
    let mut rounds = Vec::new();
    for round in input {
        let mut chars = round.chars();
        rounds.push((
            chars.nth(0).unwrap() as i8 - b'A' as i8,
            chars.nth(1).unwrap() as i8 - b'X' as i8,
        ));
    }
    let mut s = 0;
    for (opponent, you) in rounds {
        s += score(opponent, you);
    }

    s
}

#[test]
fn test_scores() {
    let tests = vec![
        (3, 0, 0),
        (4, 0, 1),
        (8, 0, 2),
        (1, 1, 0),
        (5, 1, 1),
        (9, 1, 2),
        (2, 2, 0),
        (6, 2, 1),
        (7, 2, 2),
    ];
    for (answer, a, b) in tests {
        assert_eq!(answer, score(a, b));
    }
}

#[test]
fn test() {
    let answer = 12;
    let input = vec!["A Y".to_string(), "B X".to_string(), "C Z".to_string()];
    assert_eq!(answer, _run(input));
}
