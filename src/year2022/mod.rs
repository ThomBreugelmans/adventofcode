mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn run(input: Vec<String>, day: u8) -> String {
    match day {
        1 => day1::run(input),
        2 => day2::run(input),
        3 => day3::run(input),
        4 => day4::run(input),
        5 => day5::run(input),
        6 => day6::run(input),
        7 => day7::run(input),
        8 => day8::run(input),
        9 => day9::run(input),
        10 => day10::run(input),
        11 => day11::run(input),
        12 => day12::run(input),
        13 => day13::run(input),
        14 => day14::run(input),
        15 => day15::run(input),
        16 => day16::run(input),
        _ => "[E] Unknown day...".to_string(),
    }
}
