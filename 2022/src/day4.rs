use std::fs::read_to_string;

pub fn run() {
    // get input code:
    let input: Vec<String> = {
        let mut _input = Vec::new();
        for line in read_to_string("input/day4").unwrap().lines() {
            _input.push(line.to_string());
        }
        _input
    };

    let output = _run(input);
    println!("{}", output);
}

fn _run(input: Vec<String>) -> i32 {
    let pairs = {
        let mut _pairs = Vec::new();
        for row in input {
            let numbers: Vec<i32> = row
                .split(&[',', '-'])
                .map(|e| (*e).parse::<i32>().unwrap())
                .collect();
            _pairs.push(((numbers[0], numbers[1]), (numbers[2], numbers[3])));
        }
        _pairs
    };
    let mut res = 0;
    for ((a, b), (c, d)) in pairs {
        if a >= c && b <= d {
            res += 1;
        } else if c >= a && d <= b {
            res += 1;
        }
    }

    res
}

#[test]
fn test() {
    let mut answer = "".to_string();
    let mut input = Vec::<String>::new();
    for line in read_to_string("input/test/day4").unwrap().lines() {
        if answer.is_empty() {
            answer = line.to_string();
            continue;
        }
        input.push(line.to_string());
    }
    assert_eq!(answer.parse::<i32>().unwrap(), _run(input));
}
