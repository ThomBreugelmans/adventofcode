use macros::solution;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.trim().split('\n').map(|r| r.as_bytes().into_iter().map(|&x| x-'0' as u8).collect()).collect()
}

fn find_n_batteries(batteries: &Vec<u8>, n: usize) -> u64 {
    let mut sum = 0;
    let mut si = 0usize;
    for ei in (batteries.len()-n)..batteries.len() {
        let slice = &batteries[si..=ei];
        let largest = slice.iter().max().unwrap();
        // println!("{}..={} | {}", si, ei, largest);
        si += slice.iter().position(|x| x == largest).unwrap() + 1;
        sum *= 10;
        sum += *largest as u64;
    }
    sum
}

#[solution(year=2025,day=3,part=1)]
fn run_part1(input: &str) -> String {
    let parsed = parse(input);
    let mut sum = 0u64;
    for bat in parsed {
        let r = find_n_batteries(&bat, 2);
        sum += r;
        // println!("{}", r);
    }
    format!("{}", sum)
}

#[solution(year=2025,day=3,part=2)]
fn run_part2(input: &str) -> String {
    let parsed = parse(input);
    let mut sum = 0u64;
    for bat in parsed {
        let r = find_n_batteries(&bat, 12);
        sum += r;
        // println!("{}", r);
    }
    format!("{}", sum)
}

#[allow(dead_code)]
const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

#[test]
fn test_parser() {
    let parsed = parse("0123456789");
    assert_eq!(parsed, vec!(vec!(0u8,1u8,2u8,3u8,4u8,5u8,6u8,7u8,8u8,9u8)));
}

#[test]
fn test_part1() {
    let answer = "357";
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "3121910778619";
    assert_eq!(answer, run_part2(TEST_INPUT));
}
