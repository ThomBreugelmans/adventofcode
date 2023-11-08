use std::cmp::min;
use std::collections::{HashMap, HashSet};
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
    let mut common = HashMap::new();

    for (j, rucksack) in input.iter().enumerate() {
        if j % 3 == 0 {
            common = HashMap::new();
        }
        let mut chars = HashSet::new();
        for c in rucksack.chars() {
            chars.insert(c);
        }
        for c in chars {
            common
                .entry(c)
                .and_modify(|e| *e = min(j % 3, *e + 1))
                .or_insert(0);
        }

        if j % 3 == 2 {
            common
                .iter()
                .filter_map(|(&k, &v)| if v == 2 { Some(k) } else { None })
                .for_each(|k| items.push(k));
        }
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
