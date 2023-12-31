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
        10 => solution!(year2023, day10),
        11 => solution!(year2023, day11),
        12 => solution!(year2023, day12),
        13 => solution!(year2023, day13),
        14 => solution!(year2023, day14),
        15 => solution!(year2023, day15),
        _ => panic!("[E] Unknown day..."),
    }
}
