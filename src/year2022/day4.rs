pub fn run(input: Vec<String>) -> String {
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

    res.to_string()
}

#[test]
fn test() {
    let answer = "4".to_string();
    let input = vec![
        "2-4,6-8".to_string(),
        "2-3,4-5".to_string(),
        "5-7,7-9".to_string(),
        "2-8,3-7".to_string(),
        "6-6,4-6".to_string(),
        "2-6,4-8".to_string(),
    ];
    assert_eq!(answer, run(input));
}
