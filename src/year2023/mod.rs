use aoc::solution;
use aoc::utils::solution::*;

pub fn run(day: u32) -> Solution {
    match day {
        1 => solution!(year2023, day01),
        2 => solution!(year2023, day02),
        3 => solution!(year2023, day03),
        _ => panic!("[E] Unknown day..."),
    }
}
