extern crate aoc;

use aoc::y2024::d24::wires_crossed::{define_z_output, get_swappable_node_pairs};
use aoc::run_for_n_groups;

fn main() {
    run_for_n_groups(2024, 24, "Part 1", &["sample", "input"], define_z_output);
    run_for_n_groups(2024, 24, "Part 2", &["input"], |groups| get_swappable_node_pairs(groups, 4));
}
