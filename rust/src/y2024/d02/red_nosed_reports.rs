use crate::util::parse_num::parse_i32;
use itertools::Itertools;

pub fn count_safe(levels: &[String]) -> usize {
    levels
        .iter()
        .map(|level| level.split(" ").map(parse_i32).collect_vec())
        .filter(|level| is_safe(level, true) || is_safe(level, false))
        .count()
}

pub fn count_safe_with_tolerance(levels: &[String]) -> usize {
    levels
        .iter()
        .map(|level| level.split(" ").map(parse_i32).collect_vec())
        .filter(|level| is_safe_with_tolerance(level, true) || is_safe_with_tolerance(level, false))
        .count()
}

fn is_safe_with_tolerance(level: &[i32], increasing: bool) -> bool {
    for current in 1..level.len() - 1 {
        if !is_within_bounds(level, current - 1, current, increasing) {
            return is_safe_if_removed(level, current - 1, increasing)
                || is_safe_if_removed(level, current, increasing);
        }
    }

    true
}

fn is_safe_if_removed(level: &[i32], removed: usize, increasing: bool) -> bool {
    (removed == 0 || is_within_bounds(level, removed - 1, removed + 1, increasing))
        && is_safe(&level[removed + 1..], increasing)
}

fn is_safe(level: &[i32], increasing: bool) -> bool {
    level
        .windows(2)
        .all(|distance| is_within_bounds(distance, 0, 1, increasing))
}

fn is_within_bounds(level: &[i32], left: usize, right: usize, positive: bool) -> bool {
    let diff = level[left] - level[right];

    let on_bound = diff != 0 && diff.abs() <= 3;
    let sign_matches = positive == (diff > 0);

    on_bound && sign_matches
}
