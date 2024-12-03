pub fn count_safe(levels: Vec<String>) -> usize {
    levels
        .iter()
        .map(|level| level.split(" ").map(&to_i32).collect::<Vec<i32>>())
        .filter(|level| is_safe(level, true) || is_safe(level, false))
        .count()
}

pub fn count_safe_with_tolerance(levels: Vec<String>) -> usize {
    levels
        .iter()
        .map(|level| level.split(" ").map(&to_i32).collect::<Vec<i32>>())
        .filter(|level| is_safe_with_tolerance(level, true) || is_safe_with_tolerance(level, false))
        .count()
}

fn is_safe_with_tolerance(level: &[i32], increasing: bool) -> bool {
    if level.len() < 2 {
        return true;
    }

    for higher in 1..level.len() - 1 {
        let lower = higher - 1;

        if !is_within_bounds(&level[lower..=higher], increasing) {
            let remove_lower = (lower == 0 || is_within_bounds_indexed(level, lower - 1, higher, increasing))
                && is_safe(&level[higher..], increasing);
            let remove_higher = is_within_bounds_indexed(level, lower, higher + 1, increasing)
                && is_safe(&level[higher + 1..], increasing);
            return remove_lower || remove_higher;
        }
    }

    true
}

fn is_safe(level: &[i32], increasing: bool) -> bool {
    level
        .windows(2)
        .all(|distance| is_within_bounds(distance, increasing))
}

fn is_within_bounds(sub_level: &[i32], positive: bool) -> bool {
    is_within_bounds_indexed(sub_level, 0, 1, positive)
}

fn is_within_bounds_indexed(level: &[i32], left: usize, right: usize, positive: bool) -> bool {
    let diff = level[left] - level[right];

    let on_bound = diff != 0 && diff.abs() <= 3;
    let sign_matches = positive == (diff > 0);

    on_bound && sign_matches
}

fn to_i32(num: &str) -> i32 {
    i32::from_str_radix(num, 10).unwrap()
}
