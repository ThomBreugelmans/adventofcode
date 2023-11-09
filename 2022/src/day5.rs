use std::collections::VecDeque;
use std::fs::read_to_string;

pub fn run() {
    // get input code:
    let input: Vec<String> = {
        let mut _input = Vec::new();
        for line in read_to_string("input/day5").unwrap().lines() {
            _input.push(line.to_string());
        }
        _input
    };

    let output = _run(input);
    println!("{}", output);
}

fn parse(input: Vec<String>) -> (Vec<VecDeque<char>>, Vec<(i32, i32, i32)>) {
    let mut stacks = Vec::<VecDeque<char>>::new();
    for _ in 0..((input[0].len() + 1) / 4) {
        stacks.push(VecDeque::<char>::new());
    }
    let mut input_iter = input.iter();
    let mut row = input_iter.next().unwrap();
    while !row.is_empty() {
        let mut iterator = row.chars();
        let _ = iterator.next();
        for i in 0..stacks.len() {
            let b = iterator.next().unwrap();
            let _ = iterator.nth(2);
            if b != ' ' {
                stacks[i].push_front(b);
            }
        }
        row = input_iter.next().unwrap();
    }

    // stacks have been created, now parse actions
    let mut actions = Vec::new();
    for row in input_iter {
        if row.is_empty() {
            continue;
        }
        let x = row.split(' ').collect::<Vec<&str>>();
        actions.push((
            x[1].parse::<i32>().unwrap(),
            x[3].parse::<i32>().unwrap() - 1,
            x[5].parse::<i32>().unwrap() - 1,
        ));
    }
    (stacks, actions)
}

fn _run(input: Vec<String>) -> String {
    let (mut stacks, actions) = parse(input);

    for (count, from, to) in actions {
        let mut intermediat = VecDeque::new();
        for _ in 0..count {
            let b = stacks[from as usize].pop_back().unwrap();
            intermediat.push_front(b);
        }
        stacks[to as usize].append(&mut intermediat);
    }

    String::from_iter(stacks.iter().map(|e| e.clone().pop_back().unwrap()))
}

#[test]
fn test() {
    let answer = "MCD".to_string();
    let input = vec![
        "    [D]    ".to_string(),
        "[N] [C]    ".to_string(),
        "[Z] [M] [P]".to_string(),
        " 1   2   3 ".to_string(),
        "".to_string(),
        "move 1 from 2 to 1".to_string(),
        "move 3 from 1 to 3".to_string(),
        "move 2 from 2 to 1".to_string(),
        "move 1 from 1 to 2".to_string(),
    ];
    assert_eq!(answer, _run(input));
}
