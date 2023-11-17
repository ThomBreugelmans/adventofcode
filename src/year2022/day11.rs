use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<u64>,
    op: String,
    rhs: String,
    div_by: u64,
    true_monkey: usize,
    false_monkey: usize,
}

fn parse(input: Vec<String>) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    let mut inp_iter = input.iter();
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

pub fn run(input: Vec<String>) -> String {
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
    let input = vec![
        "Monkey 0:".to_string(),
        "  Starting items: 79, 98".to_string(),
        "  Operation: new = old * 19".to_string(),
        "  Test: divisible by 23".to_string(),
        "    If true: throw to monkey 2".to_string(),
        "    If false: throw to monkey 3".to_string(),
        "".to_string(),
        "Monkey 1:".to_string(),
        "  Starting items: 54, 65, 75, 74".to_string(),
        "  Operation: new = old + 6".to_string(),
        "  Test: divisible by 19".to_string(),
        "    If true: throw to monkey 2".to_string(),
        "    If false: throw to monkey 0".to_string(),
        "".to_string(),
        "Monkey 2:".to_string(),
        "  Starting items: 79, 60, 97".to_string(),
        "  Operation: new = old * old".to_string(),
        "  Test: divisible by 13".to_string(),
        "    If true: throw to monkey 1".to_string(),
        "    If false: throw to monkey 3".to_string(),
        "".to_string(),
        "Monkey 3:".to_string(),
        "  Starting items: 74".to_string(),
        "  Operation: new = old + 3".to_string(),
        "  Test: divisible by 17".to_string(),
        "    If true: throw to monkey 0".to_string(),
        "    If false: throw to monkey 1".to_string(),
        "".to_string(),
    ];

    assert_eq!(answer, run(input));
}
