use clap::Parser;
use std::fs::read_to_string;
use std::process::exit;

mod tree;

mod year2022;

#[derive(Parser)]
struct CliArguments {
    prefix: String,
    year: u16,
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
        for line in read_to_string(format!(
            "{}input/{}/day{}",
            args.prefix, args.year, args.day
        ))
        .unwrap()
        .lines()
        {
            _input.push(line.to_string());
        }
        _input
    };

    let output = match args.year {
        2022 => year2022::run(input, args.day),
        _ => unimplemented!(),
    };

    println!("{}", output);
}
