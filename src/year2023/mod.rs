use aoc::solution;
use aoc::utils::solution::*;

pub fn run(day: u32) -> Solution {
    match day {
        1 => solution!(year2023, day01),
        _ => panic!("[E] Unknown day..."),
    }
}
