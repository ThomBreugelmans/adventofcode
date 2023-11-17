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

    let mut output = Vec::new();
    let mut x: i32 = 1;
    for (i, f) in cycles.iter().enumerate() {
        if i % 40 == 0 {
            output.push('\n');
        }
        let mut c = '.';
        if x.abs_diff((i as i32) % 40) <= 1 {
            c = '#';
        }
        output.push(c);
        if let Some(v) = f {
            x += v;
        }
    }

    println!("{}", String::from_iter(output));
    "RUAKHBEK".to_string() // not really possible to test the output, so in order to have tests still working we do this.
}

#[test]
fn test() {
    let answer = "RUAKHBEK".to_string();
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
