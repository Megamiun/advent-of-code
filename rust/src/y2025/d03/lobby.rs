use itertools::Itertools;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater};

#[allow(dead_code)]
pub fn get_sum_of_joltage(lines: &[String], batteries: usize) -> u64 {
    let banks = get_banks(lines);

    banks.iter().map(|bank| get_max_joltage(bank, batteries)).sum()
}

#[allow(dead_code)]
pub fn get_max_joltage(bank: &[u64], batteries_left: usize) -> u64 {
    if batteries_left == 0 {
        return 0;
    }
    let next_batteries_left = batteries_left - 1;

    let biggest_position = bank.iter()
        .dropping_back(next_batteries_left)
        .position_max_by(|lhs, rhs| equal_is_greater_compare(lhs, rhs))
        .unwrap();

    let bank_after_biggest = &bank[biggest_position + 1..bank.len()];

    let current_amount = bank[biggest_position] * 10u64.pow(next_batteries_left as u32);
    current_amount + get_max_joltage(bank_after_biggest, next_batteries_left)
}


// This is done so position_max_by gives us the first biggest, not the last
fn equal_is_greater_compare(lhs: &&u64, rhs: &&u64) -> Ordering {
    let comp = (*lhs).cmp(rhs);
    match comp {
        Equal => Greater,
        _ => comp
    }
}

fn get_banks(lines: &[String]) -> Vec<Vec<u64>> {
    lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect_vec())
        .collect_vec()
}
