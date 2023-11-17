use aoc::utils::solution::Solution;
use clap::Parser;
use std::process::exit;

mod year2022;

#[derive(Parser, Default)]
struct CliArguments {
    #[clap(required = false)]
    prefix: String,
    year: u16,
    day: u8,
}

fn main() {
    let args = CliArguments::parse();
    if args.day > 25 {
        println!("Only 25 days until Christmas exist!");
        exit(1);
    } else if args.day == 0 {
        println!("There is no 0th day until Christmas!");
        exit(1);
    }

    let Solution {
        year,
        day,
        input,
        wrapper,
    } = match args.year {
        2022 => year2022::run(args.day),
        _ => unimplemented!(),
    };

    println!(
        "Solution for challenge {} of year {}:\n{}",
        day,
        year,
        (wrapper)(input)
    );
}
