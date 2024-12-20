extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d03::mull_it_over::{sum_muls, sum_muls_in_do};

fn main() {
    run_for_files(2024, 3, "Part 1", &["sample", "input"], sum_muls);
    run_for_files(2024, 3, "Part 2", &["sample", "input"], sum_muls_in_do);
}
