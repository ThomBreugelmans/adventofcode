use inventory;

pub mod span;
pub mod utils;

/// The structure holding the metadata for each registered solution.
pub struct Solution {
    pub year: u16,
    pub day: u8,
    pub part: u8, // 1 or 2
    pub name: &'static str,
    pub func: fn(&str) -> String,
}

// **Crucial:** Define the inventory collection. This sets up the static registry.
inventory::collect!(Solution);

// The macro will generate code that looks like this:
// submit! {
//     Solution {
//         year: 2023, day: 1, part: 1,
//         name: "day_01_part_1",
//         func: day_01_part_1,
//     }
// }
