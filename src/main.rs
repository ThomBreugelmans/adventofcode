use arboard;
use clap::Parser;
use dotenv::dotenv;
use lib::Solution;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue, USER_AGENT};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use std::{env, fs::read_to_string, time::Instant};

// mod year2022;
// mod year2023;
mod year2025;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// year of challenge solutions to run
    #[arg(short, long)]
    year: Option<u16>,

    /// day of challenge solutions to run
    #[arg(short, long)]
    day: Option<u8>,

    /// part of challenge solutions to run
    #[arg(short, long)]
    part: Option<u8>,

    /// amount of times to run solutions (for benchmarking)
    #[arg(short, long)]
    count: Option<usize>,
}

pub fn format_duration(duration: Duration) -> String {
    // Define the thresholds in nanoseconds (n_s)
    const NANOS_PER_MICRO: f64 = 1_000.0;
    const NANOS_PER_MILLI: f64 = 1_000_000.0;
    const NANOS_PER_SECOND: f64 = 1_000_000_000.0;
    const NANOS_PER_MINUTE: f64 = 60.0 * NANOS_PER_SECOND;
    const NANOS_PER_HOUR: f64 = 60.0 * NANOS_PER_MINUTE;

    // Get the total duration in floating-point nanoseconds
    let total_nanos = duration.as_secs_f64() * 1_000_000_000.0;

    // Use a match-like structure to find the largest appropriate unit.
    // We check from the largest unit down to the smallest.
    let (value, unit) = {
        if total_nanos.abs() >= NANOS_PER_HOUR {
            (total_nanos / NANOS_PER_HOUR, "h")
        } else if total_nanos.abs() >= NANOS_PER_MINUTE {
            (total_nanos / NANOS_PER_MINUTE, "m")
        } else if total_nanos.abs() >= NANOS_PER_SECOND {
            (total_nanos / NANOS_PER_SECOND, "s")
        } else if total_nanos.abs() >= NANOS_PER_MILLI {
            (total_nanos / NANOS_PER_MILLI, "ms")
        } else if total_nanos.abs() >= NANOS_PER_MICRO {
            (total_nanos / NANOS_PER_MICRO, "μs")
        } else {
            (total_nanos, "ns") // Default to nanoseconds
        }
    };

    // Format the output. Using 2 decimal places is common for dynamic units.
    format!("{:.2} {}", value, unit)
}

fn push_to_clip(result: &String) -> Result<(), arboard::Error> {
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_text(result.clone())?;
    assert_eq!(
        result,
        &clipboard.get_text()?,
        "Expected clipboard to contain the value just pushed"
    );
    Ok(())
}

fn main() {
    dotenv().ok();
    let args = Args::parse();
    // Contains (year, day, part, result, duration)
    let mut results: Vec<(u16, u8, u8, String, String)> = vec![];

    // inventory::iter() accesses the static list of all registered Solutions
    for solution in inventory::iter::<Solution> {
        if (args.year.is_none() || solution.year == args.year.unwrap())
            && (args.day.is_none() || solution.day == args.day.unwrap())
            && (args.part.is_none() || solution.part == args.part.unwrap())
        {
            // Found specified solution, get input
            let input_fn = format!("./input/year{}/day{:02}", solution.year, solution.day);
            let input_path = PathBuf::from(&input_fn);
            if !input_path.exists() {
                // Download the input from adventofcode.com
                // Example UA="github.com/ThomBreugelmans/adventofcode by thombreugelmans@outlook.com";
                let user_agent = env::vars().find(|(k, _)| k == "AOC_USER_AGENT").unwrap().1;
                let session_cookie = env::vars().find(|(k, _)| k == "AOC_SESSION").unwrap().1;

                let mut headers = HeaderMap::new();
                headers.insert(USER_AGENT, HeaderValue::from_str(&user_agent).unwrap());
                let cookie_header =
                    HeaderValue::from_str(&format!("session={}", session_cookie)).unwrap();
                headers.insert(COOKIE, cookie_header);

                let client = reqwest::blocking::Client::new();
                let response = client
                    .get(format!(
                        "https://adventofcode.com/{}/day/{}/input",
                        solution.year, solution.day
                    ))
                    .headers(headers)
                    .send() // Await the response
                    .expect("Something went wrong while getting puzzle input"); // Check for 4xx/5xx status codes

                // if needed create all parent directories for this input file
                if let Some(parent) = input_path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let mut file = File::create(input_path).expect("Error creating puzzle input file");
                let bytes = response.bytes().expect("Error reading HTTP response body");
                file.write_all(&bytes)
                    .expect("Error writing to puzzle input file");
            }

            let input = read_to_string(input_fn)
                .expect(format!("No input found for {}-{}", solution.year, solution.day).as_str());

            let mut duration = Duration::ZERO;
            let mut result = None;
            let count = args.count.unwrap_or(1);
            for _ in 0..(count) {
                let now = Instant::now();
                result = Some((solution.func)(&input));
                duration += now.elapsed();
            }

            results.push((
                solution.year,
                solution.day,
                solution.part,
                result.unwrap(),
                format_duration(duration / count as u32),
            ));
        }
    }

    if results.is_empty() {
        return;
    }

    results.sort_by(|a, b| {
        (a.0 as u32 * 1000 + a.1 as u32 * 10 + a.2 as u32)
            .cmp(&(b.0 as u32 * 1000 + b.1 as u32 * 10 + b.2 as u32))
    });

    // Printing results in a nice table:
    // 1. First we need to get the length of longest result and duration
    let max_len_result = 6.max(results.iter().map(|v| v.3.len()).max().unwrap());
    let max_len_duration = 8.max(results.iter().map(|v| v.4.len()).max().unwrap());
    let delim = format!(
        "+{}+{}+{}+{}+{}+",
        "-".repeat(6),
        "-".repeat(5),
        "-".repeat(6),
        "-".repeat(max_len_result + 2),
        "-".repeat(max_len_duration + 2)
    );
    println!("\n{}", delim);
    println!(
        "| Year | Day | Part | Result{} | Duration{} |",
        " ".repeat(max_len_result - 6),
        " ".repeat(max_len_duration - 8)
    );
    println!("{}", delim);
    results
        .iter()
        .map(|(y, d, p, r, dur)| {
            format!(
                "| {y} | {d: <3} | {p: <4} | {r}{} | {dur}{} |",
                &" ".repeat(max_len_result - r.len()),
                &" ".repeat(max_len_duration - dur.len())
            )
        })
        .for_each(|r| println!("{}", r));
    println!("{}\n", delim);

    if args.year.is_some() && args.day.is_some() && args.part.is_some() {
        // If we have all options set (-y, -d, -p), we can put this on the clipboard, which can be used to easily paste
        // these parameters also ensure that there is only 1 result in results
        match push_to_clip(
            &results
                .last()
                .expect("Expected at least 1 result to put on clipboard")
                .3,
        ) {
            Ok(_) => println!("[i] Result copied to clipboard"),
            Err(e) => eprintln!("[E] Error copying result to clipboard:\n{e}"),
        }
    }
}
