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
        res += if a >= c && a <= d {
            1
        } else if c >= a && c <= b {
            1
        } else if b >= c && b <= d {
            1
        } else if d >= a && d <= b {
            1
        } else {
            0
        };
    }

    res
}

#[test]
fn test() {
    let answer = 4;
    let input = vec![
        "2-4,6-8".to_string(),
        "2-3,4-5".to_string(),
        "5-7,7-9".to_string(),
        "2-8,3-7".to_string(),
        "6-6,4-6".to_string(),
        "2-6,4-8".to_string(),
    ];
    assert_eq!(answer, _run(input));
}
