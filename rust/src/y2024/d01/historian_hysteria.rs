use std::collections::HashMap;

pub fn get_sum_of_distances(lines: &[String]) -> i32 {
    let (mut left, mut right) = get_columns(lines);

    // TODO Find a non mutable way to do this
    left.sort();
    right.sort();

    left.iter().zip(right)
        .map(|(l, r)| i32::abs(l - r))
        .sum()
}

pub fn get_similarity_score(lines: &[String]) -> i32 {
    let (left, right) = get_columns(lines);

    let counter = count_occurrences(&right);

    left.iter()
        .map(|num| num * *counter.get(num).unwrap_or(&0i32))
        .sum()
}

fn count_occurrences(right: &[i32]) -> HashMap<i32, i32> {
    let mut counter: HashMap<i32, i32> = HashMap::new();

    right.iter().for_each(|num| {
        if !counter.contains_key(num) {
            counter.insert(*num, 1);
        } else {
            counter.insert(*num, counter[num] + 1);
        }
    });
    counter
}

fn get_columns(lines: &[String]) -> (Vec<i32>, Vec<i32>) {
    lines
        .iter()
        .map(|line| {
            let mut items = line.split(' ');
            (to_i32(items.next().unwrap()), to_i32(items.last().unwrap()))
        })
        .unzip()
}

fn to_i32(num: &str) -> i32 {
    i32::from_str_radix(num, 10).unwrap()
}
