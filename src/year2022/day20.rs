use macros::solution;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Num {
    val: isize,
    order: usize,
}

fn parse(input: &str) -> Vec<Num> {
    input
        .lines()
        .enumerate()
        .map(|(i, v)| Num {
            val: v.parse::<isize>().unwrap(),
            order: i,
        })
        .collect()
}

fn move_by(v: &mut Vec<Num>, index: usize, by: isize) {
    let num = v.remove(index);
    let new_index = (index as isize + by).rem_euclid(v.len() as isize) as usize;
    v.insert(new_index, num);
}

fn mix(v: &mut Vec<Num>) {
    for i in 0..v.len() {
        if let Some((index, val)) = v
            .iter()
            .enumerate()
            .find_map(|vall| (vall.1.order == i).then_some((vall.0, vall.1.val)))
        {
            move_by(v, index, val);
        }
    }
}

#[solution(year = 2022, day = 20, part = 1)]
fn run_part1(input: &str) -> String {
    let mut nums = parse(input);

    mix(&mut nums);

    if let Some(zero_index) = nums
        .iter()
        .enumerate()
        .find_map(|(i, v)| (v.val == 0).then_some(i))
    {
        (nums[(zero_index + 1000) % nums.len()].val
            + nums[(zero_index + 2000) % nums.len()].val
            + nums[(zero_index + 3000) % nums.len()].val)
            .to_string()
    } else {
        unreachable!()
    }
}

const DECRYPTION_KEY: isize = 811_589_153;
#[solution(year = 2022, day = 20, part = 2)]
fn run_part2(input: &str) -> String {
    let mut nums = parse(input);
    nums.iter_mut().for_each(|n| n.val *= DECRYPTION_KEY);

    for _ in 0..10 {
        mix(&mut nums);
    }

    let zero_index = nums.iter().position(|n| n.val == 0).unwrap();
    (nums[(zero_index + 1000) % nums.len()].val
        + nums[(zero_index + 2000) % nums.len()].val
        + nums[(zero_index + 3000) % nums.len()].val)
        .to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "1
2
-3
3
-2
0
4";

#[test]
fn move_test() {
    let mut input = vec![
        Num {
            val: 811589153,
            order: 0,
        },
        Num {
            val: 1623178306, // % = 1
            order: 1,
        },
        Num {
            val: -2434767459, // % = 2
            order: 2,
        },
        Num {
            val: 2434767459, // % = 5
            order: 3,
        },
        Num {
            val: -1623178306, // % = 6
            order: 4,
        },
        Num { val: 0, order: 5 }, // % = 0
        Num {
            val: 3246356612, // % = 2
            order: 6,
        },
    ];
    let answer = vec![
        0,
        -2434767459,
        3246356612,
        -1623178306,
        2434767459,
        1623178306,
        811589153,
    ];

    for _ in 0..1 {
        mix(&mut input);
    }

    assert_eq!(answer, input.iter().map(|v| v.val).collect::<Vec<isize>>());
}

#[test]
fn test_part1() {
    let answer = "3";
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "1623178306";
    assert_eq!(answer, run_part2(TEST_INPUT));
}
