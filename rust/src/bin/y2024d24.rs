extern crate aoc;

use aoc::y2024::d24::wires_crossed::define_z_output;
use aoc::run_for_groups;

fn main() {
    run_for_groups(2024, 24, "Part 1", &["sample", "input"], define_z_output);
    // run_for_files(2024, 24, "Part 2", &["sample", "input"], get_biggest_lan);
}
