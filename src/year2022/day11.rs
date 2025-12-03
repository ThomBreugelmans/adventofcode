use macros::solution;
use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<u64>,
    op: String,
    rhs: String,
    div_by: u64,
    true_monkey: usize,
    false_monkey: usize,
}

fn parse(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    let mut inp_iter = input.lines();
    while let Some(_) = inp_iter.next() {
        let items = inp_iter
            .next()
            .unwrap()
            .get(18..)
            .unwrap_or("")
            .split(", ")
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        let operation_s = inp_iter.next().unwrap();
        let op = operation_s.get(23..24).unwrap().to_string();
        let rhs = operation_s.get(25..).unwrap().to_string();
        let div_by = inp_iter
            .next()
            .unwrap()
            .get(21..)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let true_monkey = inp_iter
            .next()
            .unwrap()
            .get(29..)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let false_monkey = inp_iter
            .next()
            .unwrap()
            .get(30..)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let monkey = Monkey {
            items,
            op,
            rhs,
            div_by,
            true_monkey,
            false_monkey,
        };
        inp_iter.next();
        monkeys.push(monkey);
    }
    monkeys
}

#[solution(year = 2022, day = 11, part = 2)]
pub fn run(input: &str) -> String {
    let mut monkeys = parse(input);

    let modulo = monkeys.iter().map(|m| m.div_by).product::<u64>();
    let mut counts = Vec::new();
    for _ in &monkeys {
        counts.push(0usize);
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let mut item = monkeys[i].items.pop_front().unwrap();
                item = {
                    let r = match monkeys[i].rhs.as_ref() {
                        "old" => item,
                        d => d.parse::<u64>().unwrap(),
                    };
                    match monkeys[i].op.as_ref() {
                        "*" => item * r,
                        _ => item + r,
                    }
                } % modulo;
                let to_monkey = {
                    if item % monkeys[i].div_by == 0 {
                        monkeys[i].true_monkey
                    } else {
                        monkeys[i].false_monkey
                    }
                };
                monkeys[to_monkey].items.push_back(item);

                counts[i] += 1;
            }
        }
    }

    counts.sort();
    counts.reverse();
    (counts[0] * counts[1]).to_string()
}

#[test]
fn test() {
    let answer = "2713310158".to_string();
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1

";

    assert_eq!(answer, run(input));
}
