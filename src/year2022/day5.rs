use std::collections::VecDeque;

fn parse(input: &str) -> (Vec<VecDeque<char>>, Vec<(i32, i32, i32)>) {
    let mut stacks = Vec::<VecDeque<char>>::new();
    let mut input_iter = input.lines().peekable();
    for _ in 0..((input_iter.peek().unwrap().len() + 1) / 4) {
        stacks.push(VecDeque::<char>::new());
    }

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

pub fn run(input: &str) -> String {
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
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(answer, run(input));
}
