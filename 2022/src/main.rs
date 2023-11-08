use clap::Parser;
use std::process::exit;

mod day1;
mod day2;

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

    match args.day {
        1 => day1::run(),
        2 => day2::run(),
        _ => unimplemented!(),
    };
}
