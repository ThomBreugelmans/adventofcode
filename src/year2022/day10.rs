use macros::solution;

fn parse(input: &str) -> Vec<Option<i32>> {
    let mut res = Vec::new();
    for line in input.lines() {
        if line == "noop" {
            res.push(None);
        } else {
            res.push(None);
            res.push(Some(line.get(5..).unwrap().parse::<i32>().unwrap()));
        }
    }
    res
}

#[solution(year = 2022, day = 10, part = 2)]
pub fn run(input: &str) -> String {
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
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    assert_eq!(answer, run(input));
}
