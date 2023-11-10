use std::cmp::min;
use std::collections::{HashMap, HashSet};

fn priority(item: char) -> i32 {
    (if item >= 'a' {
        (item as u8 - b'a') + 1
    } else {
        (item as u8 - b'A') + 27
    }) as i32
}

pub fn run(input: Vec<String>) -> String {
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

    items.iter().map(|&e| priority(e)).sum::<i32>().to_string()
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
    let answer = "70".to_string();
    let input = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
        "PmmdzqPrVvPwwTWBwg".to_string(),
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
        "ttgJtRGJQctTZtZT".to_string(),
        "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
    ];
    assert_eq!(answer, run(input));
}
