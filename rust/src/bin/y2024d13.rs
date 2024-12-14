extern crate aoc;

use aoc::y2024::d13::claw_contraption::{calculate, calculate_with_error};
use aoc::run_for_groups;

fn main() {
    run_for_groups(2024, 13, &["sample", "input"], &calculate);
    run_for_groups(2024, 13, &["sample", "input"], &calculate_with_error);
}
