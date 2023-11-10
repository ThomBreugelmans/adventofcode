use clap::Parser;
use std::fs::read_to_string;
use std::process::exit;

mod tree;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

#[derive(Parser)]
struct CliArguments {
    day: u8,
}

fn main() {
    let args = CliArguments::parse();
    if args.day > 25 {
        println!("Days until Christmas only run until 25!");
        exit(1);
    } else if args.day == 0 {
        println!("There is no 0th day until Christmas!");
        exit(1);
    }

    // get input code:
    let input: Vec<String> = {
        let mut _input = Vec::new();
        for line in read_to_string(format!("input/day{}", args.day))
            .unwrap()
            .lines()
        {
            _input.push(line.to_string());
        }
        _input
    };

    let output = match args.day {
        1 => day1::run(input),
        2 => day2::run(input),
        3 => day3::run(input),
        4 => day4::run(input),
        5 => day5::run(input),
        6 => day6::run(input),
        7 => day7::run(input),
        _ => unimplemented!(),
    };

    println!("{}", output);
}
