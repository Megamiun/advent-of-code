extern crate aoc;

use aoc::run_for_groups;
use aoc::y2024::d19::linen_layout::{get_possible_towels, get_towels_arrangements};

fn main() {
    run_for_groups(2024, 19, "pt1", &["sample", "input"], get_possible_towels);
    run_for_groups(2024, 19, "pt2", &["sample", "input"], get_towels_arrangements);
}
