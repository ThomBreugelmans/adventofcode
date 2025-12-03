use macros::solution;

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: usize,
    red: u32,
    green: u32,
    blue: u32,
}

fn parse(input: &str) -> Vec<Game> {
    let mut games = Vec::new();
    for line in input.lines() {
        let mut game = Game {
            id: games.len() + 1,
            red: 0,
            green: 0,
            blue: 0,
        };
        let data = line
            .trim()
            .split([':', ';', ','])
            .map(|section| section.trim());
        for d in data {
            let temp = d.split(' ').collect::<Vec<&str>>();
            match (temp[0], temp[1]) {
                (_, "red") => game.red = game.red.max(temp[0].parse::<u32>().unwrap()),
                (_, "green") => game.green = game.green.max(temp[0].parse::<u32>().unwrap()),
                (_, "blue") => game.blue = game.blue.max(temp[0].parse::<u32>().unwrap()),
                _ => (),
            }
        }
        games.push(game);
    }
    games
}

#[solution(year = 2023, day = 2, part = 1)]
fn run_part1(input: &str) -> String {
    let games = parse(input);
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;
    games
        .into_iter()
        .filter_map(|game| {
            if game.red > MAX_RED || game.green > MAX_GREEN || game.blue > MAX_BLUE {
                None
            } else {
                Some(game.id as u32)
            }
        })
        .sum::<u32>()
        .to_string()
}

#[solution(year = 2023, day = 2, part = 2)]
fn run_part2(input: &str) -> String {
    let games = parse(input);
    games
        .into_iter()
        .map(|game| game.red * game.blue * game.green)
        .sum::<u32>()
        .to_string()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

#[test]
fn test_parsing() {
    let answer = vec![
        Game {
            id: 1,
            red: 4,
            green: 2,
            blue: 6,
        },
        Game {
            id: 2,
            red: 1,
            green: 3,
            blue: 4,
        },
        Game {
            id: 3,
            red: 20,
            green: 13,
            blue: 6,
        },
        Game {
            id: 4,
            red: 14,
            green: 3,
            blue: 15,
        },
        Game {
            id: 5,
            red: 6,
            green: 3,
            blue: 2,
        },
    ];
    assert_eq!(answer, parse(TEST_INPUT));
}

#[test]
fn test_part1() {
    let answer = "8";
    assert_eq!(answer, run_part1(TEST_INPUT));
}

#[test]
fn test_part2() {
    let answer = "2286";
    assert_eq!(answer, run_part2(TEST_INPUT));
}
