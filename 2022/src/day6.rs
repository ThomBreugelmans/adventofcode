use std::collections::HashSet;
use std::fs::read_to_string;

pub fn run() {
    // get input code:
    let input: Vec<String> = {
        let mut _input = Vec::new();
        for line in read_to_string("input/day6").unwrap().lines() {
            _input.push(line.to_string());
        }
        _input
    };

    let output = _run(input);
    println!("{}", output);
}

fn _run(input: Vec<String>) -> i32 {
    let string = input
        .iter()
        .rfold("".to_string(), |_, b| b.clone())
        .chars()
        .collect::<Vec<char>>();

    for i in 0..string.len() - 14 {
        let mut set = HashSet::new();
        for j in i..i + 14 {
            set.insert(string[j]);
        }
        if set.len() == 14 {
            return i as i32 + 14;
        }
    }

    string.len() as i32 + 1
}

#[test]
fn test() {
    let mut answer = "".to_string();
    let mut input = Vec::<String>::new();
    for line in read_to_string("input/test/day6").unwrap().lines() {
        if answer.is_empty() {
            answer = line.to_string();
            continue;
        }
        input.push(line.to_string());
    }
    assert_eq!(answer.parse::<i32>().unwrap(), _run(input));
}
