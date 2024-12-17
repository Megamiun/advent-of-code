extern crate aoc;

use aoc::y2024::d17::chronospatial_computer::{execute, find};
use aoc::run_for_groups;

fn main() {
    run_for_groups(2024, 17, &["sample", "input"], &execute);
    run_for_groups(2024, 17, &["sample2"], &find); // TODO Not efficient enough for input yet
}
