use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Vector {
    a: i32,
    b: i32,
}

#[derive(Debug)]
struct Segment {
    start: i32,
    end: i32,
}

impl Segment {
    fn extend(&self, other: &Segment) -> Option<Segment> {
        if (other.start < self.start || other.start > self.end)
            && (other.end < self.start || other.end > self.end)
        {
            None
        } else {
            Some(Segment {
                start: min(self.start, other.start),
                end: max(self.end, other.end),
            })
        }
    }
}

fn manhattan(a: (i32, i32), b: (i32, i32)) -> i32 {
    (max(a.0, b.0) - min(a.0, b.0)).abs() + (max(a.1, b.1) - min(a.1, b.1)).abs()
}

fn parse(input: &Vec<String>) -> Vec<((i32, i32), (i32, i32))> {
    let iter = input.iter();
    iter.map(|i| {
        i.split(", ")
            .flat_map(|ii| ii.split(':').map(|iii| iii.split('=')))
            .flatten()
            .filter_map(|f| {
                if f.chars()
                    .map(|c| c.is_numeric() || c == '-')
                    .rfold(true, |a, b| a && b)
                {
                    Some(f.parse::<i32>().unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<i32>>()
    })
    .map(|i| ((i[0], i[1]), (i[2], i[3])))
    .collect::<Vec<((i32, i32), (i32, i32))>>()
}

fn create_segments(input: Vec<((i32, i32), (i32, i32))>) -> HashMap<i32, Vec<Segment>> {
    let mut segments = HashMap::new();
    for (sensor, beacon) in input {
        let man_d = manhattan(sensor, beacon);
        for x_o in 0..=man_d {
            let y_o = man_d - x_o;
            for (start, end) in [
                (
                    (sensor.0 - x_o, sensor.1 - y_o),
                    (sensor.0 + x_o, sensor.1 - y_o),
                ),
                (
                    (sensor.0 - x_o, sensor.1 + y_o),
                    (sensor.0 + x_o, sensor.1 + y_o),
                ),
            ] {
                let mut cur_seg = Segment {
                    start: start.0,
                    end: end.0,
                };

                if segments.contains_key(&start.1) {
                    let mut new_vec = Vec::new();
                    for segment in segments.remove(&start.1).unwrap() {
                        if let Some(ns) = cur_seg.extend(&segment) {
                            cur_seg = ns;
                        } else {
                            new_vec.push(segment);
                        }
                    }
                    new_vec.push(cur_seg);
                    segments.insert(start.1, new_vec);
                } else {
                    segments.insert(start.1, vec![cur_seg]);
                }
            }
        }
    }
    segments
}

pub fn run(input: Vec<String>) -> String {
    format!(
        "{}\n{}",
        run_part1(&input, 2_000_000),
        run_part2(&input, 4_000_000)
    )
}

fn run_part1(input: &Vec<String>, line: i32) -> String {
    let parsed = parse(input);

    // the lines are / and \ with the bottom point on the line of the sensor
    let mut sensors = HashSet::new();
    let mut beacons = HashSet::new();

    for (sen, bea) in &parsed {
        beacons.insert(*bea);
        sensors.insert(*sen);
    }

    let segments = create_segments(parsed);

    let count = segments
        .into_iter()
        .filter(|(y, _)| *y == line)
        .map(|(_, ss)| ss.into_iter().rfold(0, |a, s| a + (s.end - s.start)))
        .sum::<i32>();
    count.to_string()
}

fn run_part2(input: &Vec<String>, limit: i32) -> String {
    let parsed = parse(input);

    let segments = create_segments(parsed);

    let row = segments
        .iter()
        .filter_map(|(y, ss)| {
            if *y <= limit && *y >= 0 {
                Some((
                    *y,
                    ss.iter()
                        .map(|s| Segment {
                            start: max(0, s.start),
                            end: min(limit, s.end),
                        })
                        .rfold(0, |a, s| a + (s.end - s.start))
                        + 1,
                    ss,
                ))
            } else {
                None
            }
        })
        .reduce(|a, (y, c, ss)| if c < limit { (y, c, ss) } else { a })
        .unwrap();
    let x = (0..=limit)
        .fold(None, |a, x| {
            if a.is_none() && !row.2.iter().any(|s| x >= s.start && x <= s.end) {
                Some(x)
            } else {
                a
            }
        })
        .unwrap();

    (x as u64 * 4_000_000u64 + row.0 as u64).to_string()
}

#[test]
fn test_part1() {
    let answer = "26".to_string();
    let input = vec![
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".to_string(),
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16".to_string(),
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3".to_string(),
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16".to_string(),
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16".to_string(),
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16".to_string(),
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10".to_string(),
        "Sensor at x=2, y=0: clopsest beacon is at x=2, y=10".to_string(),
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10".to_string(),
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17".to_string(),
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22".to_string(),
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3".to_string(),
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3".to_string(),
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3".to_string(),
    ];
    assert_eq!(answer, run_part1(&input, 10))
}

#[test]
fn test_part2() {
    let answer = "56000011".to_string();
    let input = vec![
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".to_string(),
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16".to_string(),
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3".to_string(),
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16".to_string(),
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16".to_string(),
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16".to_string(),
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10".to_string(),
        "Sensor at x=2, y=0: clopsest beacon is at x=2, y=10".to_string(),
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10".to_string(),
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17".to_string(),
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22".to_string(),
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3".to_string(),
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3".to_string(),
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3".to_string(),
    ];
    assert_eq!(answer, run_part2(&input, 20))
}
