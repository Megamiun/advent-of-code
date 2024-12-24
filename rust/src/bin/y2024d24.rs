extern crate aoc;

use aoc::run_for_n_groups;
use aoc::y2024::d24::wires_crossed::define_z_output;
use aoc::y2024::d24::wires_crossed_adder::get_swappable_node_pairs_adder;

fn main() {
    run_for_n_groups(2024, 24, "Part 1", &["sample", "input"], define_z_output);
    run_for_n_groups(2024, 24, "Part 2", &["input"], |groups| get_swappable_node_pairs_adder(groups, 4));
}
