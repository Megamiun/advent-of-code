extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d07::bridge_repair::{part2, part1};

fn main() {
    run_for_files(2024, 7, &["sample", "input"], &part1);
    run_for_files(2024, 7, &["sample", "input"], &part2);
}
