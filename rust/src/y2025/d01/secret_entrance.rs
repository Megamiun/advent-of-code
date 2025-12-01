use crate::util::parse_num::parse_i32;
use itertools::Itertools;
use num_traits::ToPrimitive;

#[allow(dead_code)]
pub fn get_dials_at_zero(lines: &[String]) -> usize {
    let movements = get_movements(lines);

    movements.iter().scan(50i32, |acc, movement| {
        *acc = match movement.0 {
            'L' => (*acc - movement.1).rem_euclid(100),
            'R' => (*acc + movement.1).rem_euclid(100),
            _ => *acc
        };

        Some(*acc)
    }).filter(|num| *num == 0)
        .count()
}

#[allow(dead_code)]
pub fn get_clicks_at_zero(lines: &[String]) -> usize {
    let movements = get_movements(lines);

    let valid_range = 0..100;

    movements.iter().scan(50i32, |acc, movement| {
        let was_zero = *acc == 0;
        *acc = match movement.0 {
            'L' => *acc - movement.1,
            'R' => *acc + movement.1,
            _ => *acc
        };

        if valid_range.contains(acc) {
            return match *acc {
                0 => Some(1),
                _ => Some(0)
            }
        }

        let negative_wrap = if !was_zero && acc.is_negative() { 1 } else { 0 };
        let wraps = acc.abs() / 100;

        *acc = acc.rem_euclid(100);

        Some(wraps + negative_wrap)
    }).map(|num| num.to_usize().unwrap()).sum()
}

fn get_movements(lines: &[String]) -> Vec<(char, i32)> {
    lines
        .iter()
        .map(|line| (line.chars().next().unwrap(), parse_i32(&line[1..line.len()])))
        .collect_vec()
}
