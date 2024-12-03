extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d02::red_nosed_reports::{count_safe, count_safe_with_tolerance};

fn main() {
    run_for_files(2024, 2, &["sample", "input"], &count_safe);
    run_for_files(2024, 2, &["sample", "input"], &count_safe_with_tolerance);
}
