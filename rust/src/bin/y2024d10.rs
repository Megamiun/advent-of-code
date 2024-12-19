extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d10::hoof_it::{get_reachable_sum_for, get_trails_sum_for};

fn main() {
    run_for_files(2024, 10, "pt1", &["sample", "sample2", "input"], get_reachable_sum_for);
    run_for_files(2024, 10, "pt2", &["sample", "sample2", "input"], get_trails_sum_for);
}
