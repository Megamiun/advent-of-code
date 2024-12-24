extern crate aoc;

use aoc::y2024::d19::linen_layout::{get_possible_towels, get_towels_arrangements};
use aoc::run_for_n_groups;

fn main() {
    run_for_n_groups(2024, 19, "Part 1", &["sample", "input"], get_possible_towels);
    run_for_n_groups(2024, 19, "Part 2", &["sample", "input"], get_towels_arrangements);
}
