extern crate aoc;

use aoc::y2024::d16::reindeer_maze::{get_min_path, get_unique_best_spots};
use aoc::run_for_files;

fn main() {
    run_for_files(2024, 16, &["sample", "sample2", "input"], &get_min_path);
    run_for_files(2024, 16, &["sample", "sample2", "input"], &get_unique_best_spots);
}
