extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d22::monkey_market::{get_max_bananas_after_4_numbers, get_sum_of_secrets_after};

fn main() {
    run_for_files(2024, 22, "Part 1", &["sample", "input"], |lines| get_sum_of_secrets_after(lines, 2001));
    run_for_files(2024, 22, "Part 2", &["sample2", "input"], |lines| get_max_bananas_after_4_numbers(lines));
}
