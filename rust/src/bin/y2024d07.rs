extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d07::bridge_repair::{with_basic_operators, with_extra_operators};

fn main() {
    run_for_files(2024, 7, "pt1", &["sample", "input"], with_basic_operators);
    run_for_files(2024, 7, "pt2", &["sample", "input"], with_extra_operators);
}
