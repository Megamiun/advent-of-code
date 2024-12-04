use std::cmp::max;
use std::collections::HashMap;
use std::hash::Hash;
use y2023::d02::cube_conumdrum::Color::{B, G, R};

#[derive(PartialEq, Eq, Hash)]
enum Color {
    R,
    G,
    B,
}

impl Color {
    const RED: &'static str = "red";
    const GREEN: &'static str = "green";
    const BLUE: &'static str = "blue";

    const VALUES: &'static [Color] = &[R, G, B];

    fn value(&self) -> &'static str {
        match self {
            R => Color::RED,
            G => Color::GREEN,
            B => Color::BLUE,
        }
    }
}

pub fn sum_minimum_power(rounds: &Vec<String>) -> u32 {
    rounds
        .iter()
        .map(|round| get_game_summary(round))
        .map(|(_, round)| get_minimum_for_colors(round))
        .map(|values| values.iter().fold(1, |acc, curr| acc * curr.1))
        .sum::<u32>()
}

fn get_minimum_for_colors(rounds: Vec<HashMap<&Color, u32>>) -> HashMap<&Color, u32> {
    rounds.iter().fold(HashMap::new(), |acc, curr| {
        let mut map = HashMap::new();

        Color::VALUES
            .iter()
            .map(|key| (key, *max(acc.get(key), curr.get(key)).unwrap_or(&0u32)))
            .filter(|(_, value)| *value != 0)
            .for_each(|(key, val)| {
                map.insert(key, val);
            });

        map
    })
}

pub fn sum_valid(original: &Vec<String>, limits: &Vec<String>) -> u32 {
    let mut limits_num = limits.iter().map(|t| u32::from_str_radix(t, 10).unwrap());

    let limit_per_color = HashMap::from([
        (&R, limits_num.nth(0).unwrap()),
        (&G, limits_num.nth(0).unwrap()),
        (&B, limits_num.nth(0).unwrap()),
    ]);

    original
        .iter()
        .map(|line| get_game_summary(line))
        .filter(|(_, rounds)| is_within_limit(rounds, &limit_per_color))
        .map(|(value, _)| value)
        .sum()
}

fn is_within_limit(maps: &Vec<HashMap<&Color, u32>>, limits: &HashMap<&Color, u32>) -> bool {
    maps.iter()
        .all(|round| round.iter().all(|(color, number)| number <= &limits[color]))
}

fn get_game_summary(line: &str) -> (u32, Vec<HashMap<&Color, u32>>) {
    let mut split_line = line.split(":");

    let (game, rounds) = (split_line.next().unwrap(), split_line.next().unwrap());
    let game_num = game
        .split(" ")
        .nth(1)
        .map(|num| u32::from_str_radix(num, 10).unwrap())
        .unwrap();

    let round_summary = rounds
        .split(";")
        .map(|round| {
            round
                .split(",")
                .map(|color_data| get_round_summary(color_data))
                .collect()
        })
        .collect();

    (game_num, round_summary)
}

fn get_round_summary(color_data: &str) -> (&Color, u32) {
    let mut split_color_data = color_data.trim().split(" ");
    let (number, color_name) = (
        u32::from_str_radix(split_color_data.next().unwrap(), 10).unwrap(),
        split_color_data.next().unwrap(),
    );

    (
        Color::VALUES
            .iter()
            .filter(|color| color.value() == color_name)
            .next()
            .unwrap(),
        number,
    )
}
