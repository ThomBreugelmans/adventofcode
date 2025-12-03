use macros::solution;
use std::collections::HashMap;
use std::iter::{Copied, Cycle, Enumerate, Peekable};
use std::slice::Iter;

type Wrapper<'a, T> = Cycle<Copied<Iter<'a, T>>>;
const FLOOR: u8 = 0xff;
const WALLS: u32 = 0x01010101;
const ROCKS: [Rock; 5] = [
    Rock {
        size: 1,
        shape: 0x0000003c,
    },
    Rock {
        size: 3,
        shape: 0x00103810,
    },
    Rock {
        size: 3,
        shape: 0x00080838,
    },
    Rock {
        size: 4,
        shape: 0x20202020,
    },
    Rock {
        size: 2,
        shape: 0x00003030,
    },
];

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Rock {
    size: usize,
    shape: u32,
}

#[derive(Clone)]
struct State<'a> {
    rocks: Wrapper<'a, Rock>,
    pub jets: Peekable<Enumerate<Wrapper<'a, u8>>>,
    pub tower: Vec<u8>,
    pub height: usize,
    heights: [usize; 7],
}

impl State<'_> {
    fn new(input: &[u8]) -> State<'_> {
        let mut state = State {
            rocks: ROCKS.iter().copied().cycle(),
            jets: input.iter().copied().cycle().enumerate().peekable(),
            tower: vec![0; 13_000], // each cycle, in worst case is 13 high, we SWAG and say 5000 iterations is more than enough rows
            height: 0,
            heights: [0; 7],
        };
        state.tower[0] = FLOOR;
        state
    }

    fn get_relief(&self) -> [usize; 7] {
        let mut relief = self.heights;
        (0..relief.len()).for_each(|i| {
            relief[i] = self.height - relief[i];
        });
        //dbg!(relief);
        relief
    }
}

type Height = usize;
type NumJets = usize;
impl Iterator for State<'_> {
    type Item = (Height, NumJets);

    fn next(&mut self) -> Option<Self::Item> {
        let Rock { size, mut shape } = self.rocks.next().unwrap();
        let mut chunk = WALLS;
        let mut index = self.height + 3;

        loop {
            let (_, jet) = self.jets.next().unwrap();
            let candidate = if jet == b'<' {
                shape.rotate_left(1)
            } else {
                shape.rotate_right(1)
            };

            if candidate & chunk == 0 {
                // there is no overlap
                shape = candidate;
            }

            // this way we can check all rows of the falling rock with the walls and the existing rocks
            chunk = (chunk << 8) | WALLS | (self.tower[index] as u32);

            if shape & chunk == 0 {
                // there is no overlap, so the rock can keep falling
                index -= 1;
            } else {
                // there is overlap, place the rock in the tower
                let bytes = shape.to_le_bytes();
                self.tower[index + 1] |= bytes[0];
                self.tower[index + 2] |= bytes[1];
                self.tower[index + 3] |= bytes[2];
                self.tower[index + 4] |= bytes[3];
                // current rock couldve fallen inbetween existing rocks, so we take the max between the top of the rock and the current tower height.
                self.height = self.height.max(index + size);
                // update relief
                for offset in 1..=7 {
                    for x in 1..=4 {
                        if self.tower[index + x] & (1 << offset) != 0 {
                            self.heights[offset - 1] = self.heights[offset - 1].max(index + x);
                        }
                    }
                }
                break;
            }
        }

        Some((self.height, self.jets.peek().unwrap().0 - 1))
    }
}

#[solution(year = 2022, day = 17, part = 1)]
fn run_part1(input: &str) -> String {
    State::new(input.trim().as_bytes())
        .nth(2021)
        .unwrap()
        .0
        .to_string()
}

#[solution(year = 2022, day = 17, part = 2)]
fn run_part2(input: &str) -> String {
    // now need to run for 1_000_000_000_000 iterations, so we need to do some cycle detection
    let directions = input.trim().as_bytes();
    let mut state = State::new(directions);
    let mut seen = HashMap::new();

    let mut total_rocks = 0;
    let mut highest = 0;
    let mut jet;
    let mut heights = [0usize; 13_000];
    while total_rocks < 1_000_000_000_000 {
        (highest, jet) = state.next().unwrap();
        total_rocks += 1;
        heights[total_rocks] = state.height;
        let key = (
            state.get_relief(),
            total_rocks % ROCKS.len(),
            jet % directions.len(),
        );
        if seen.contains_key(&key) {
            // we have a cycle
            let round = *seen.get(&key).unwrap();

            let start_height = heights[round];
            let cycle_height = heights[total_rocks] - start_height;
            let cycle_count = (1_000_000_000_000 - round) / (total_rocks - round);
            let skipped_rocks = cycle_count * (total_rocks - round);
            let remainder = 1_000_000_000_000 - skipped_rocks;
            let rem_height = heights[remainder] - start_height;
            total_rocks += skipped_rocks + remainder;
            highest += cycle_height * (cycle_count - 1) + rem_height;
        } else {
            seen.insert(key, total_rocks);
        }
    }

    highest.to_string()
}

#[test]
fn test_relief() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    let mut state = State::new(input.trim().as_bytes());
    for _ in 0..10 {
        state.next();
        dbg!(state.get_relief());
    }
}

#[test]
fn test_part1() {
    let answer = "3068".to_string();
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(answer, run_part1(input));
}

#[test]
fn test_part2() {
    // cycle should occur on 28 and is 35 long
    let answer = "1514285714288".to_string();
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(answer, run_part2(input));
}
