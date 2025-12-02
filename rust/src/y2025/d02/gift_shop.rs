use crate::util::parse_num::parse_u64;
use itertools::Itertools;

#[allow(dead_code)]
pub fn get_numbers_with_repeated_digits(lines: &[String]) -> u64 {
    let ranges = get_ranges(lines);

    ranges.iter().flat_map(|range|
        (range.0..=range.1)
            .filter(|&num| is_number_repeated(num, get_decimal_length(num), 2))
    ).sum()
}

#[allow(dead_code)]
pub fn get_numbers_with_repeated_digits_boundless(lines: &[String]) -> u64 {
    let ranges = get_ranges(lines);

    ranges.iter().flat_map(|range|
        (range.0..=range.1).filter(|&num| {
            let num_length = get_decimal_length(num);

            (2..=num_length)
                .any(|fragments| is_number_repeated(num, num_length, fragments))
        })
    ).sum()
}

fn is_number_repeated(num: u64, num_length: u32, fragments: u32) -> bool {
    if num_length % fragments != 0 {
        return false;
    }

    let fragment_multiplier = 10u64.pow(num_length / fragments);
    let remainder = num % fragment_multiplier;

    let expected = (0..fragments)
        .fold(0, |acc, _| (acc * fragment_multiplier) + remainder);

    expected == num
}

fn get_decimal_length(number: u64) -> u32 {
    1 + number.ilog10()
}

fn get_ranges(lines: &[String]) -> Vec<(u64, u64)> {
    lines
        .iter()
        .flat_map(|line| line.split(',').collect_vec())
        .map(|range| range.splitn(2, '-').map(|num| parse_u64(num)).next_tuple().unwrap())
        .collect_vec()
}
