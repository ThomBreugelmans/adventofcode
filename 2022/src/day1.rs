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
    let mut answer = "".to_string();
    let mut input = Vec::<String>::new();
    for line in read_to_string("input/test/day1").unwrap().lines() {
        if answer.is_empty() {
            answer = line.to_string();
            continue;
        }
        input.push(line.to_string());
    }
    assert_eq!(answer.parse::<i32>().unwrap(), _run(input));
}
