fn parse(input: Vec<String>) -> Vec<Option<i32>> {
    let mut res = Vec::new();
    for line in input {
        if line == "noop" {
            res.push(None);
        } else {
            res.push(None);
            res.push(Some(line.get(5..).unwrap().parse::<i32>().unwrap()));
        }
    }
    res
}

pub fn run(input: Vec<String>) -> String {
    let cycles = parse(input);

    let mut frequency_strength = 0;
    let mut x = 1;
    for (i, f) in cycles.iter().enumerate() {
        if i % 40 == 19 {
            frequency_strength += (i + 1) as i32 * x;
        }
        if let Some(v) = f {
            x += v;
        }
    }

    frequency_strength.to_string()
}

#[test]
fn test() {
    let answer = "13140".to_string();
    let input = vec![
        "addx 15".to_string(),
        "addx -11".to_string(),
        "addx 6".to_string(),
        "addx -3".to_string(),
        "addx 5".to_string(),
        "addx -1".to_string(),
        "addx -8".to_string(),
        "addx 13".to_string(),
        "addx 4".to_string(),
        "noop".to_string(),
        "addx -1".to_string(),
        "addx 5".to_string(),
        "addx -1".to_string(),
        "addx 5".to_string(),
        "addx -1".to_string(),
        "addx 5".to_string(),
        "addx -1".to_string(),
        "addx 5".to_string(),
        "addx -1".to_string(),
        "addx -35".to_string(),
        "addx 1".to_string(),
        "addx 24".to_string(),
        "addx -19".to_string(),
        "addx 1".to_string(),
        "addx 16".to_string(),
        "addx -11".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 21".to_string(),
        "addx -15".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx -3".to_string(),
        "addx 9".to_string(),
        "addx 1".to_string(),
        "addx -3".to_string(),
        "addx 8".to_string(),
        "addx 1".to_string(),
        "addx 5".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx -36".to_string(),
        "noop".to_string(),
        "addx 1".to_string(),
        "addx 7".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 2".to_string(),
        "addx 6".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 1".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 7".to_string(),
        "addx 1".to_string(),
        "noop".to_string(),
        "addx -13".to_string(),
        "addx 13".to_string(),
        "addx 7".to_string(),
        "noop".to_string(),
        "addx 1".to_string(),
        "addx -33".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 2".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 8".to_string(),
        "noop".to_string(),
        "addx -1".to_string(),
        "addx 2".to_string(),
        "addx 1".to_string(),
        "noop".to_string(),
        "addx 17".to_string(),
        "addx -9".to_string(),
        "addx 1".to_string(),
        "addx 1".to_string(),
        "addx -3".to_string(),
        "addx 11".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 1".to_string(),
        "noop".to_string(),
        "addx 1".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx -13".to_string(),
        "addx -19".to_string(),
        "addx 1".to_string(),
        "addx 3".to_string(),
        "addx 26".to_string(),
        "addx -30".to_string(),
        "addx 12".to_string(),
        "addx -1".to_string(),
        "addx 3".to_string(),
        "addx 1".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx -9".to_string(),
        "addx 18".to_string(),
        "addx 1".to_string(),
        "addx 2".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 9".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx -1".to_string(),
        "addx 2".to_string(),
        "addx -37".to_string(),
        "addx 1".to_string(),
        "addx 3".to_string(),
        "noop".to_string(),
        "addx 15".to_string(),
        "addx -21".to_string(),
        "addx 22".to_string(),
        "addx -6".to_string(),
        "addx 1".to_string(),
        "noop".to_string(),
        "addx 2".to_string(),
        "addx 1".to_string(),
        "noop".to_string(),
        "addx -10".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 20".to_string(),
        "addx 1".to_string(),
        "addx 2".to_string(),
        "addx 2".to_string(),
        "addx -6".to_string(),
        "addx -11".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
    ];

    assert_eq!(answer, run(input));
}
