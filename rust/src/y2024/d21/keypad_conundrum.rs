use crate::util::Index2D;
use crate::y2024::util::direction::Direction::{Down, Left, Right, Up};
use std::cmp::min_by_key;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::LazyLock;

pub fn get_sum_of_complexity(lines: &[String]) -> usize {
    lines.iter().map(get_complexity).sum()
}

fn get_complexity(line: &String) -> usize {
    let numerical_part = line.strip_suffix('A').unwrap();
    let numerical_value = usize::from_str_radix(numerical_part, 10).unwrap();
    let minimum = solve_for(numerical_part, &[NUMERIC.deref(), DIRECTIONAL.deref(), DIRECTIONAL.deref()]);
    
    minimum.len() * numerical_value
}
fn solve_for(line: &str, map: &[&HashMap<char, Index2D>]) -> String {
    if map.is_empty() {
        return line.to_string() + "A"
    }

    ("A".to_string() + line + "A").chars().collect::<Vec<_>>().windows(2).map(|window| {
        let [horizontal, vertical] = derive_movement(window, map[0]);

        if BANNED.contains(&(window[1], window[0])) {
            return solve_for(&join_into(&vertical, &horizontal), &map[1..])
        } else if BANNED.contains(&(window[0], window[1])) {
            return solve_for(&join_into(&horizontal, &vertical), &map[1..])
        }

        min_by_key(
            solve_for(&join_into(&horizontal, &vertical), &map[1..]),
            solve_for(&join_into(&vertical, &horizontal), &map[1..]),
            |str| str.len()
        )
    }).collect()
}

fn join_into(horizontal: &str, vertical: &str) -> String {
    [vertical, horizontal].join("")
}

fn derive_movement(window: &[char], position: &HashMap<char, Index2D>) -> [String; 2] {
    let distance = position[&window[1]] - position[&window[0]];

    let horizontal = if distance.0 > 0 { Right } else { Left };
    let vertical = if distance.1 > 0 { Down } else { Up };

    [
        String::from_iter((0..distance.0.abs()).map(|_| horizontal.into_char())),
        String::from_iter((0..distance.1.abs()).map(|_| vertical.into_char()))
    ]
}

const NUMERIC: LazyLock<HashMap<char, Index2D>> = LazyLock::new(|| HashMap::from([
    ('A', Index2D(2, 3)),
    ('0', Index2D(1, 3)),
    ('1', Index2D(0, 2)),
    ('2', Index2D(1, 2)),
    ('3', Index2D(2, 2)),
    ('4', Index2D(0, 1)),
    ('5', Index2D(1, 1)),
    ('6', Index2D(2, 1)),
    ('7', Index2D(0, 0)),
    ('8', Index2D(1, 0)),
    ('9', Index2D(2, 0)),
]));

const DIRECTIONAL: LazyLock<HashMap<char, Index2D>> = LazyLock::new(|| HashMap::from([
    ('<', Index2D(0, 1)),
    ('v', Index2D(1, 1)),
    ('>', Index2D(2, 1)),
    ('^', Index2D(1, 0)),
    ('A', Index2D(2, 0)),
]));

const BANNED: &[(char, char)] = &[
    ('A', '1'),
    ('A', '4'),
    ('A', '7'),
    ('0', '1'),
    ('0', '4'),
    ('0', '7'),
    ('A', '<'),
    ('^', '<'),
];