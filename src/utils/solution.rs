#[derive(Debug)]
pub struct Solution {
    pub year: &'static str,
    pub day: &'static str,
    pub input: &'static str,
    pub wrapper: fn(&str) -> String,
}

#[macro_export]
macro_rules! solution {
    ($year:tt, $day:tt) => {
        Solution {
            year: (&stringify!($year)),
            day: (&stringify!($day)),
            input: include_str!(concat![
                "../../input/",
                stringify!($year),
                "/",
                stringify!($day),
            ]),
            wrapper: |raw: &str| {
                use aoc::$year::$day::*;
                let output = run(raw).to_string();
                output
            },
        }
    };
}
