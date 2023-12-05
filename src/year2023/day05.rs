use std::num::ParseIntError;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

struct Almanac {
    seeds: Vec<(usize, usize)>,
    seed_to_soil: Vec<(usize, usize, usize)>,
    soil_to_fert: Vec<(usize, usize, usize)>,
    fert_to_water: Vec<(usize, usize, usize)>,
    water_to_light: Vec<(usize, usize, usize)>,
    light_to_temp: Vec<(usize, usize, usize)>,
    temp_to_hum: Vec<(usize, usize, usize)>,
    hum_to_loc: Vec<(usize, usize, usize)>,
}

impl Almanac {
    fn seed_to_loc(&self, seed: usize) -> usize {
        let soil = find_dest(&self.seed_to_soil, seed);
        let fert = find_dest(&self.soil_to_fert, soil);
        let water = find_dest(&self.fert_to_water, fert);
        let light = find_dest(&self.water_to_light, water);
        let temp = find_dest(&self.light_to_temp, light);
        let hum = find_dest(&self.temp_to_hum, temp);
        let loc = find_dest(&self.hum_to_loc, hum);
        /*println!(
            "{} -> {} -> {} -> {} -> {} -> {} -> {} -> {}",
            seed, soil, fert, water, light, temp, hum, loc
        );*/
        loc
    }
}

fn seeds(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    preceded(
        tag("seeds: "),
        separated_list1(
            space1::<&str, nom::error::Error<&str>>,
            separated_pair(digit1, space1, digit1),
        ),
    )(input)
}

fn mapper(preceeded: &str) -> impl FnMut(&str) -> IResult<&str, Vec<(&str, &str, &str)>> + '_ {
    move |input: &str| {
        preceded(
            preceded(multispace0, tag(preceeded)),
            separated_list1(
                line_ending::<&str, nom::error::Error<&str>>,
                tuple((digit1, preceded(space1, digit1), preceded(space1, digit1))),
            ),
        )(input)
    }
}

fn parse(input: &str) -> Almanac {
    let (input, seeds_) = seeds(input).unwrap();
    let (input, seed_to_soil_) = mapper("seed-to-soil map:\n")(input).unwrap();
    let (input, soil_to_fert_) = mapper("soil-to-fertilizer map:\n")(input).unwrap();
    let (input, fertilizer_to_water_) = mapper("fertilizer-to-water map:\n")(input).unwrap();
    let (input, water_to_light_) = mapper("water-to-light map:\n")(input).unwrap();
    let (input, light_to_temp_) = mapper("light-to-temperature map:\n")(input).unwrap();
    let (input, temp_to_hum_) = mapper("temperature-to-humidity map:\n")(input).unwrap();
    let (_, hum_to_loc_) = mapper("humidity-to-location map:\n")(input).unwrap();

    fn vec_tuple_str2usize(v: Vec<(&str, &str, &str)>) -> Vec<(usize, usize, usize)> {
        fn tuple_str2usize((a, b, c): (&str, &str, &str)) -> (usize, usize, usize) {
            (
                a.parse::<usize>().unwrap(),
                b.parse::<usize>().unwrap(),
                c.parse::<usize>().unwrap(),
            )
        }
        v.into_iter().map(tuple_str2usize).collect::<Vec<_>>()
    }

    Almanac {
        seeds: seeds_
            .into_iter()
            .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
            .collect(),
        seed_to_soil: vec_tuple_str2usize(seed_to_soil_),
        soil_to_fert: vec_tuple_str2usize(soil_to_fert_),
        fert_to_water: vec_tuple_str2usize(fertilizer_to_water_),
        water_to_light: vec_tuple_str2usize(water_to_light_),
        light_to_temp: vec_tuple_str2usize(light_to_temp_),
        temp_to_hum: vec_tuple_str2usize(temp_to_hum_),
        hum_to_loc: vec_tuple_str2usize(hum_to_loc_),
    }
}

pub fn run(input: &str) -> String {
    format!("Part 1: {}\nPart 2: {}", run_part1(input), run_part2(input))
}

fn find_dest(v: &Vec<(usize, usize, usize)>, val: usize) -> usize {
    for (dest, source, len) in v {
        if val >= *source && val < (source + len) {
            return dest + (val - source);
        }
    }
    val
}

fn run_part1(input: &str) -> String {
    let almanac = parse(input);

    let locs = almanac
        .seeds
        .clone()
        .into_iter()
        .flat_map(|s| vec![almanac.seed_to_loc(s.0), almanac.seed_to_loc(s.1)]);

    locs.min().unwrap().to_string()
}

fn run_part2(input: &str) -> String {
    let almanac = parse(input);

    let locs = almanac.seeds.clone().into_iter().flat_map(|ss| {
        (ss.0..ss.0 + ss.1)
            .map(|s| almanac.seed_to_loc(s))
            .collect::<Vec<_>>()
    });

    locs.min().unwrap().to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[test]
fn test_par1() {
    let answer = "35";
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "46";
    assert_eq!(answer, run_part2(TEST_INPUT));
}
