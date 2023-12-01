use aoc::solution;
use aoc::utils::solution::*;

pub fn run(day: u32) -> Solution {
    match day {
        1 => solution!(year2022, day1),
        2 => solution!(year2022, day2),
        3 => solution!(year2022, day3),
        4 => solution!(year2022, day4),
        5 => solution!(year2022, day5),
        6 => solution!(year2022, day6),
        7 => solution!(year2022, day7),
        8 => solution!(year2022, day8),
        9 => solution!(year2022, day9),
        10 => solution!(year2022, day10),
        11 => solution!(year2022, day11),
        12 => solution!(year2022, day12),
        13 => solution!(year2022, day13),
        14 => solution!(year2022, day14),
        15 => solution!(year2022, day15),
        16 => solution!(year2022, day16),
        17 => solution!(year2022, day17),
        18 => solution!(year2022, day18),
        19 => solution!(year2022, day19),
        _ => panic!("[E] Unknown day..."),
    }
}
