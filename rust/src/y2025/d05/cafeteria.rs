use crate::util::parse_num::parse_u64;
use itertools::Itertools;
use std::cmp::max;
use std::ops::RangeInclusive;

#[allow(dead_code)]
pub fn get_fresh_count(lines: &[&[String];2]) -> usize {
    let ranges = get_ranges(lines[0]);

    get_ingredients(lines[1]).iter()
        .filter(|ingredient| ranges.iter().any(|range| range.contains(ingredient)))
        .count()
}

#[allow(dead_code)]
pub fn get_whole_fresh_ingredients(lines: &[&[String];2]) -> u64 {
    merge_intersections(get_ranges(lines[0])).iter()
        .map(|range| (range.end() - range.start()) + 1)
        .sum()
}

fn merge_intersections(ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let sorted_ranges = ranges.iter()
        .sorted_by(|a, b| Ord::cmp(a.start(), b.start()))
        .collect_vec();

    let result = sorted_ranges.iter()
        .fold(((*sorted_ranges[0]).clone(), vec![]), |(curr_range, acc), range| {
            if curr_range.end() >= range.start() {
                (*curr_range.start()..=*(max(range.end(), curr_range.end())), acc)
            } else {
                ((*range).clone(), vec![acc, vec![curr_range]].concat())
            }
        });

    vec![result.1, vec![result.0]].concat()
}

fn get_ingredients(lines: &[String]) -> Vec<u64> {
    lines.iter().map(|num| parse_u64(num)).collect_vec()
}

fn get_ranges(lines: &[String]) -> Vec<RangeInclusive<u64>> {
    lines.iter()
        .map(|line| line.split("-").map(|num| parse_u64(num)).collect_vec())
        .map(|range| range[0]..=range[1])
        .collect_vec()
}
