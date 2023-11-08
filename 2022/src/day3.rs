use std::collections::HashMap;
use std::fs::read_to_string;

pub fn run() {
    // get input code:
    let input: Vec<String> = {
        let mut _input = Vec::new();
        for line in read_to_string("input/day3").unwrap().lines() {
            _input.push(line.to_string());
        }
        _input
    };

    let output = _run(input);
    println!("{}", output);
}

fn priority(item: char) -> i32 {
    (if item >= 'a' {
        (item as u8 - b'a') + 1
    } else {
        (item as u8 - b'A') + 27
    }) as i32
}

fn _run(input: Vec<String>) -> i32 {
    let mut items = Vec::new();
    for rucksack in input {
        let mut common = HashMap::new();
        for (i, c) in rucksack.chars().enumerate() {
            if i < (rucksack.len() / 2) {
                common.insert(c, false);
            } else {
                common.entry(c).and_modify(|e| *e = true);
            }
        }

        common
            .iter()
            .filter_map(|(&k, &v)| if v { Some(k) } else { None })
            .for_each(|k| items.push(k));
    }

    items.iter().map(|&e| priority(e)).sum::<i32>()
}

#[test]
fn test_priority() {
    let tests = vec![
        (16, 'p'),
        (38, 'L'),
        (42, 'P'),
        (22, 'v'),
        (20, 't'),
        (19, 's'),
    ];
    for (answer, c) in tests {
        assert_eq!(answer, priority(c));
    }
}

#[test]
fn test() {
    let mut answer = "".to_string();
    let mut input = Vec::<String>::new();
    for line in read_to_string("input/test/day3").unwrap().lines() {
        if answer.is_empty() {
            answer = line.to_string();
            continue;
        }
        input.push(line.to_string());
    }
    assert_eq!(answer.parse::<i32>().unwrap(), _run(input));
}
