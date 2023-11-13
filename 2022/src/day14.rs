use std::cmp::{max, min};
use std::collections::HashSet;

fn parse(input: Vec<String>) -> (HashSet<(i32, i32)>, i32) {
    let mut h = HashSet::new();
    let mut lowest = i32::MIN;
    for line in input {
        let iter = line.split(" -> ");
        let lines = iter
            .map(|a| {
                let mut e = a.split(',');
                (
                    e.next().unwrap().parse::<i32>().unwrap(),
                    e.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .collect::<Vec<(i32, i32)>>();
        let mut iter2 = lines.iter();
        let mut point = iter2.next().unwrap();
        for temp in iter2 {
            for x in min(point.0, temp.0)..=max(point.0, temp.0) {
                h.insert((x, point.1));
            }
            for y in min(point.1, temp.1)..=max(point.1, temp.1) {
                h.insert((point.0, y));
                if y > lowest {
                    lowest = y;
                }
            }
            point = temp;
        }
    }
    (h, lowest)
}

pub fn run(input: Vec<String>) -> String {
    let (rocks, lowest) = parse(input);
    let mut sands = HashSet::<(i32, i32)>::new();

    let mut sandpos = (500, 0);
    let mut count = 0;
    while sands.get(&(500, 0)).is_none() {
        let mut broke = false;
        for v in [
            (sandpos.0, sandpos.1 + 1),
            (sandpos.0 - 1, sandpos.1 + 1),
            (sandpos.0 + 1, sandpos.1 + 1),
        ] {
            if v.1 < lowest + 2 && rocks.get(&v).is_none() && sands.get(&v).is_none() {
                sandpos = v;
                broke = true;
                break;
            }
        }
        if !broke {
            sands.insert(sandpos);
            sandpos = (500, 0);
            count += 1;
        }
    }

    count.to_string()
}

#[test]
fn test() {
    let answer = "93".to_string();
    let input = vec![
        "498,4 -> 498,6 -> 496,6".to_string(),
        "503,4 -> 502,4 -> 502,9 -> 494,9".to_string(),
    ];
    assert_eq!(answer, run(input));
}
