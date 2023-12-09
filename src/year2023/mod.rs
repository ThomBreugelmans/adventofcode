use aoc::solution;
use aoc::utils::solution::*;

pub fn run(day: u32) -> Solution {
    match day {
        1 => solution!(year2023, day01),
        2 => solution!(year2023, day02),
        3 => solution!(year2023, day03),
        4 => solution!(year2023, day04),
        5 => solution!(year2023, day05),
        6 => solution!(year2023, day06),
        7 => solution!(year2023, day07),
        8 => solution!(year2023, day08),
        9 => solution!(year2023, day09),
        _ => panic!("[E] Unknown day..."),
    }
}
