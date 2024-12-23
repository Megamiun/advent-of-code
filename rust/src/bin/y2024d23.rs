extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d23::lan_party::{get_biggest_lan, get_lans_with_t};

fn main() {
    run_for_files(2024, 23, "Part 1", &["sample", "input"], get_lans_with_t);
    run_for_files(2024, 23, "Part 2", &["sample", "input"], get_biggest_lan);
}
