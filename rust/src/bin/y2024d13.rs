extern crate aoc;

use aoc::run_for_groups;
use aoc::y2024::d13::claw_contraption::{calculate, calculate_with_error};

fn main() {
    run_for_groups(2024, 13, "Part 1", &["sample", "input"], calculate);
    run_for_groups(2024, 13, "Part 2", &["sample", "input"], calculate_with_error);
}
