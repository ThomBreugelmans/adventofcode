use aoc::solution;
use aoc::utils::solution::*;

pub fn run(day: u32) -> Solution {
    match day {
        1 => solution!(year2025, day01),
        2 => solution!(year2025, day02),
        _ => panic!("[E] Unknown day..."),
    }
}
