#[derive(Debug)]
struct Number {
    value: u32,
    x_min: usize,
    x_max: usize,
    y: usize,
}

#[derive(Debug)]
struct Symbol {
    x: usize,
    y: usize,
    is_gear: bool,
}

impl Number {
    fn is_adjacent(
        &self,
        Symbol {
            x: s_x,
            y: s_y,
            is_gear: _,
        }: &Symbol,
    ) -> bool {
        *s_x >= (self.x_min.saturating_sub(1))
            && *s_x <= (self.x_max + 1)
            && *s_y >= self.y.saturating_sub(1)
            && *s_y <= (self.y + 1)
    }
}

fn parse(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut res = (Vec::new(), Vec::new());
    let mut num = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ss) in line.chars().enumerate() {
            let mut push_num = false;
            if ss.is_numeric() {
                num.push(ss);
            } else if ss != '.' {
                res.1.push(Symbol {
                    x,
                    y,
                    is_gear: ss == '*',
                });
                push_num = true;
            } else {
                push_num = true;
            }

            if push_num && !num.is_empty() {
                res.0.push(Number {
                    value: String::from_iter(num.clone()).parse::<u32>().unwrap(),
                    x_min: x.saturating_sub(num.len()),
                    x_max: x - 1,
                    y,
                });
                num.clear();
            }
        }
        if !num.is_empty() {
            res.0.push(Number {
                value: String::from_iter(num.clone()).parse::<u32>().unwrap(),
                x_min: line.len().saturating_sub(num.len()),
                x_max: line.len() - 1,
                y,
            });
            num.clear();
        }
    }
    res
}

pub fn run(input: &str) -> String {
    format!("Part 1: {}\nPart 2: {}", run_part1(input), run_part2(input))
}

fn run_part1(input: &str) -> String {
    let (parts, symbols) = parse(input);
    parts
        .into_iter()
        .filter(|p| symbols.iter().any(|s| p.is_adjacent(s)))
        .map(|p| p.value)
        .sum::<u32>()
        .to_string()
}

fn run_part2(input: &str) -> String {
    let (parts, symbols) = parse(input);
    symbols
        .into_iter()
        .filter_map(|s| {
            let n = parts.iter().filter(|p| p.is_adjacent(&s));
            if s.is_gear && n.clone().count() >= 2 {
                Some(n.map(|p| p.value).product::<u32>())
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

#[test]
fn test_is_adjacent() {
    let symbols = vec![
        Symbol {
            x: 3,
            y: 1,
            is_gear: false,
        },
        Symbol {
            x: 6,
            y: 3,
            is_gear: false,
        },
        Symbol {
            x: 3,
            y: 4,
            is_gear: false,
        },
        Symbol {
            x: 5,
            y: 5,
            is_gear: false,
        },
        Symbol {
            x: 3,
            y: 8,
            is_gear: false,
        },
        Symbol {
            x: 5,
            y: 8,
            is_gear: false,
        },
    ];

    let num = Number {
        value: 58,
        x_min: 7,
        x_max: 8,
        y: 5,
    };

    assert!(!symbols.iter().any(|s| num.is_adjacent(s)));
}

#[test]
fn test_part1() {
    let answer = "4361";
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "467835";
    assert_eq!(answer, run_part2(TEST_INPUT));
}
