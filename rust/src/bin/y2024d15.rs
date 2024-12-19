extern crate aoc;

use aoc::run_for_groups;
use aoc::y2024::d15::warehouse_woes::{move_robot_single, move_robot_wide};

fn main() {
    run_for_groups(2024, 15, "pt1", &["sample-small", "sample", "input"], move_robot_single);
    run_for_groups(2024, 15, "pt2", &["sample-small-2", "sample", "input"], move_robot_wide);
}
