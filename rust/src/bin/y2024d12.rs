extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d12::garden_groups::{find_price_by_perimeter, find_price_by_sides};

fn main() {
    run_for_files(2024, 12, "pt1", &["sample-small", "sample", "input"], find_price_by_perimeter);
    run_for_files(2024, 12, "pt2", &["sample-small", "sample", "input"], find_price_by_sides);
}
