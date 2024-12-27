use crate::util::parse_num::parse_i32;
use itertools::Itertools;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn get_sum_of_distances(lines: &[String]) -> i32 {
    let (mut left, mut right) = get_columns(lines);

    left.sort();
    right.sort();

    left.iter().zip(right)
        .map(|(l, r)| i32::abs(l - r))
        .sum()
}

#[allow(dead_code)]
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
        *counter.entry(*num).or_insert(0) += 1;
    });
    counter
}

fn get_columns(lines: &[String]) -> (Vec<i32>, Vec<i32>) {
    lines
        .iter()
        .map(|line| line.split("   ").map(parse_i32).collect_tuple().unwrap())
        .unzip()
}
