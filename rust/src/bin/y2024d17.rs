extern crate aoc;

use aoc::run_for_groups;
use aoc::y2024::d17::chronospatial_computer::{execute, find};

fn main() {
    run_for_groups(2024, 17, "pt1", &["sample", "input"], execute);
    run_for_groups(2024, 17, "pt2", &["sample2"], find); // TODO Not efficient enough for input yet
}
