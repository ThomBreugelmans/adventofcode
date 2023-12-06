use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult, Parser,
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
    fn seed_to_loc(&self, seed: usize, range: usize) -> Vec<(usize, usize)> {
        vec![(seed, range)]
            .into_iter()
            .flat_map(|r| range_mapper(r, &self.seed_to_soil))
            .flat_map(|r| range_mapper(r, &self.soil_to_fert))
            .flat_map(|r| range_mapper(r, &self.fert_to_water))
            .flat_map(|r| range_mapper(r, &self.water_to_light))
            .flat_map(|r| range_mapper(r, &self.light_to_temp))
            .flat_map(|r| range_mapper(r, &self.temp_to_hum))
            .flat_map(|r| range_mapper(r, &self.hum_to_loc))
            .collect::<Vec<_>>()
    }
}

fn seeds(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    preceded(
        tag("seeds: "),
        separated_list1(
            space1::<&str, nom::error::Error<&str>>,
            separated_pair(digit1, space1, digit1),
        )
        .map(|v| {
            v.into_iter()
                .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                .collect::<Vec<_>>()
        }),
    )(input)
}

fn mapper(preceeded: &str) -> impl FnMut(&str) -> IResult<&str, Vec<(usize, usize, usize)>> + '_ {
    move |input: &str| {
        preceded(
            preceded(multispace0, tag(preceeded)),
            separated_list1(
                line_ending::<&str, nom::error::Error<&str>>,
                tuple((digit1, preceded(space1, digit1), preceded(space1, digit1))),
            )
            .map(|v| {
                v.into_iter()
                    .map(|(a, b, c)| {
                        (
                            a.parse::<usize>().unwrap(),
                            b.parse::<usize>().unwrap(),
                            c.parse::<usize>().unwrap(),
                        )
                    })
                    .collect::<Vec<_>>()
            }),
        )(input)
    }
}

fn parse(input: &str) -> Almanac {
    let (input, seeds) = seeds(input).unwrap();
    let (input, seed_to_soil) = mapper("seed-to-soil map:\n")(input).unwrap();
    let (input, soil_to_fert) = mapper("soil-to-fertilizer map:\n")(input).unwrap();
    let (input, fert_to_water) = mapper("fertilizer-to-water map:\n")(input).unwrap();
    let (input, water_to_light) = mapper("water-to-light map:\n")(input).unwrap();
    let (input, light_to_temp) = mapper("light-to-temperature map:\n")(input).unwrap();
    let (input, temp_to_hum) = mapper("temperature-to-humidity map:\n")(input).unwrap();
    let (_, hum_to_loc) = mapper("humidity-to-location map:\n")(input).unwrap();

    Almanac {
        seeds,
        seed_to_soil,
        soil_to_fert,
        fert_to_water,
        water_to_light,
        light_to_temp,
        temp_to_hum,
        hum_to_loc,
    }
}

fn range_mapper(from: (usize, usize), to: &Vec<(usize, usize, usize)>) -> Vec<(usize, usize)> {
    let mut new_ranges = Vec::new();
    let mut cur_ranges = vec![from];
    while let Some((from, len)) = cur_ranges.pop() {
        let mut pushed = false;
        for (dest, source, len2) in to {
            if from >= *source && from < (source + len2) {
                // we start somewhere in the range
                let new_start = from - source + dest;
                if (from + len) > *source && (from + len) <= source + len2 {
                    // we are wholly contained within the range
                    new_ranges.push((new_start, len));
                } else {
                    // the mapper will cut off our range
                    let contained_len = len2 - (from - source);
                    new_ranges.push((new_start, contained_len));
                    cur_ranges.push((source + len2, len - contained_len));
                }
                pushed = true;
                break;
            } else if (from + len) > *source && (from + len) <= (source + len2) {
                // the mapper will cut off our range
                let contained_len = (from + len) - source;
                new_ranges.push((*dest, contained_len));
                cur_ranges.push((from, len - contained_len));
                pushed = true;
                break;
            }
        }
        if !pushed {
            new_ranges.push((from, len));
        }
    }
    new_ranges
}

pub fn run(input: &str) -> String {
    format!("Part 1: {}\nPart 2: {}", run_part1(input), run_part2(input))
}

fn run_part1(input: &str) -> String {
    let almanac = parse(input);

    let locs = almanac
        .seeds
        .clone()
        .into_iter()
        .flat_map(|s| {
            let mut x = almanac.seed_to_loc(s.0, 1);
            x.append(&mut almanac.seed_to_loc(s.1, 1));
            x
        })
        .map(|l| l.0);

    locs.min().unwrap().to_string()
}

fn run_part2(input: &str) -> String {
    let almanac = parse(input);

    almanac
        .seeds
        .clone()
        .into_iter()
        .flat_map(|(s, l)| almanac.seed_to_loc(s, l))
        .reduce(|a, b| if b.0 < a.0 { b } else { a })
        .unwrap()
        .0
        .to_string()
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
fn test_range_mapper() {
    let input = (79, 14);
    let mapper = vec![(50, 98, 2), (52, 50, 48)];
    assert_eq!(vec![(81, 14)], range_mapper(input, &mapper));
}

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
