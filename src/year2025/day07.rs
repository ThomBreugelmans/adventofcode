use macros::solution;

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .split('\n')
        .map(|x| x.chars().collect())
        .collect()
}

#[solution(year = 2025, day = 7, part = 1)]
fn part1(input: &str) -> String {
    let parsed = parse(input);
    let start_pos = parsed[0].iter().position(|c| *c == 'S').unwrap();
    let mut tachyon_beams = vec![start_pos];
    let mut sum = 0u64;
    for row in parsed {
        tachyon_beams = tachyon_beams
            .into_iter()
            .flat_map(|beam| match row[beam] {
                '^' => {
                    sum += 1;
                    [Some(beam - 1), Some(beam + 1)]
                }
                _ => [Some(beam), None],
            })
            .filter_map(|b| b)
            .fold(Vec::new(), |mut vec, beam| {
                if !vec.contains(&beam) {
                    vec.push(beam);
                }
                vec
            });
    }
    sum.to_string()
}

#[solution(year = 2025, day = 7, part = 2)]
fn part2(input: &str) -> String {
    let parsed = parse(input);
    let start_pos = parsed[0].iter().position(|c| *c == 'S').unwrap();
    let mut tachyon_beams = vec![(start_pos, 1)];
    for row in parsed {
        tachyon_beams = tachyon_beams
            .into_iter()
            .flat_map(|(beam, count)| match row[beam] {
                '^' => [Some((beam - 1, count)), Some((beam + 1, count))],
                _ => [Some((beam, count)), None],
            })
            .filter_map(|b| b)
            .fold(Vec::new(), |mut vec, beams| {
                if let Some(pos) = vec.iter().position(|other_beam| other_beam.0 == beams.0) {
                    vec[pos] = (beams.0, vec[pos].1 + beams.1);
                } else {
                    vec.push(beams);
                }
                vec
            });
    }
    tachyon_beams
        .into_iter()
        .map(|beam| beam.1)
        .sum::<u64>()
        .to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

#[test]
fn test_part1() {
    let answer = "21";
    assert_eq!(answer, part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "40";
    assert_eq!(answer, part2(TEST_INPUT));
}
