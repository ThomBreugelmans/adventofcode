use macros::solution;

#[solution(year = 2022, day = 4, part = 2)]
pub fn run(input: &str) -> String {
    let pairs = {
        let mut _pairs = Vec::new();
        for row in input.lines() {
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

    res.to_string()
}

#[test]
fn test() {
    let answer = "4".to_string();
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(answer, run(input));
}
