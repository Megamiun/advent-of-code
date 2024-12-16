extern crate aoc;

use aoc::run_for_groups;
use aoc::y2024::d15::warehouse_woes::{pt1, pt2};

fn main() {
    run_for_groups(2024, 15, &["sample-small", "sample", "input"], &pt1);
    run_for_groups(2024, 15, &["sample-small-2", "sample", "input"], &pt2);
}
