extern crate aoc;

use aoc::y2024::d15::warehouse_woes::{move_robot_single, move_robot_wide};
use aoc::run_for_n_groups;

fn main() {
    run_for_n_groups(2024, 15, "Part 1", &["sample-small", "sample", "input"], move_robot_single);
    run_for_n_groups(2024, 15, "Part 2", &["sample-small-2", "sample", "input"], move_robot_wide);
}
