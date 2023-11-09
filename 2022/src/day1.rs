use std::fs::read_to_string;

pub fn run() {
    // get input code:
    let input: Vec<String> = {
        let mut _input = Vec::new();
        for line in read_to_string("input/day1").unwrap().lines() {
            _input.push(line.to_string());
        }
        _input
    };

    let output = _run(input);
    println!("{}", output);
}

fn _run(input: Vec<String>) -> i32 {
    let mut elves: Vec<i32> = vec![0];
    for cal in input {
        if cal.is_empty() {
            elves.push(0);
            continue;
        }
        let cal_i = cal.parse::<i32>().unwrap();
        if let Some(last) = elves.last_mut() {
            *last += cal_i;
        }
    }

    elves.sort();
    elves.reverse();
    elves[0] + elves[1] + elves[2]
}

#[test]
fn test() {
    let answer = 45000;
    let input = vec![
        "1000".to_string(),
        "2000".to_string(),
        "3000".to_string(),
        "".to_string(),
        "4000".to_string(),
        "".to_string(),
        "5000".to_string(),
        "6000".to_string(),
        "".to_string(),
        "7000".to_string(),
        "8000".to_string(),
        "9000".to_string(),
        "".to_string(),
        "10000".to_string(),
    ];
    assert_eq!(answer, _run(input));
}
