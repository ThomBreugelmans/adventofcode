use macros::solution;
use itertools::Itertools;
use std::collections::HashMap;

fn parse(input: &str) -> Vec<(&str, Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(' ').expect("Could not split at <space>");
            (
                a,
                b.split(',').map(|v| v.parse::<usize>().unwrap()).collect(),
            )
        })
        .collect()
}

fn recurse(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    springs: &[u8],
    within: Option<usize>,
    nums: &[usize],
) -> usize {
    if springs.is_empty() {
        return match (within, nums.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == nums[0] => 1,
            _ => 0,
        };
    }
    if within.is_some() && nums.is_empty() {
        return 0;
    }

    let key = (springs.len(), within.unwrap_or(0), nums.len());
    if let Some(&x) = cache.get(&key) {
        return x;
    }

    let ways = match (springs[0], within) {
        (b'.', Some(x)) if x != nums[0] => 0,
        (b'.', Some(_)) => recurse(cache, &springs[1..], None, &nums[1..]),
        (b'.', None) => recurse(cache, &springs[1..], None, nums),
        (b'#', Some(_)) => recurse(cache, &springs[1..], within.map(|x| x + 1), nums),
        (b'#', None) => recurse(cache, &springs[1..], Some(1), nums),
        (b'?', Some(x)) => {
            // if x is some value and x is not equal to nums[0] then this needs to become a #
            // if x is some value and x is equal to nums[0] then this needs to become a .
            let mut a = recurse(cache, &springs[1..], within.map(|x| x + 1), nums);
            if x == nums[0] {
                a += recurse(cache, &springs[1..], None, &nums[1..])
            }
            a
        }
        (b'?', None) => {
            recurse(cache, &springs[1..], Some(1), nums) + recurse(cache, &springs[1..], None, nums)
        }
        _ => unreachable!(),
    };

    cache.insert(key, ways);
    ways
}

#[solution(year=2023, day=12, part=1)]
fn run_part1(input: &str) -> String {
    let parsed = parse(input);
    let mut cache = HashMap::new();
    parsed
        .into_iter()
        .map(|r| {
            cache.clear();
            recurse(&mut cache, r.0.as_bytes(), None, &r.1)
        })
        .sum::<usize>()
        .to_string()
}

#[solution(year=2023, day=12, part=2)]
fn run_part2(input: &str) -> String {
    let parsed = parse(input);
    let mut cache = HashMap::new();
    parsed
        .into_iter()
        .map(|r| {
            let expanded_springs = (0..5).map(|_| r.0).join("?");
            let expanded_nums = (0..5).flat_map(|_| &r.1).copied().collect::<Vec<_>>();
            cache.clear();
            recurse(
                &mut cache,
                expanded_springs.as_bytes(),
                None,
                &expanded_nums,
            )
        })
        .sum::<usize>()
        .to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

#[test]
fn test_part1() {
    let answer = "21";
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "525152";
    assert_eq!(answer, run_part2(TEST_INPUT));
}
