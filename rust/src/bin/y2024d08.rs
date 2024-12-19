extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d08::resonant_collinearity::{get_antinodes_for_repeated_jumps, get_antinodes_for_single_jump};

fn main() {
    run_for_files(2024, 8, "pt1", &["sample", "input"], get_antinodes_for_single_jump);
    run_for_files(2024, 8, "pt2", &["sample", "input"], get_antinodes_for_repeated_jumps);
}
