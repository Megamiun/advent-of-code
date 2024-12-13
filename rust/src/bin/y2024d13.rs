extern crate aoc;

use aoc::y2024::d13::claw_contraption::{pt1, pt2};
use aoc::run_for_groups;

fn main() {
    run_for_groups(2024, 13, &["sample", "input"], &pt1);
    run_for_groups(2024, 13, &["sample", "input"], &pt2);
}
