extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d01::historian_hysteria::{get_similarity_score, get_sum_of_distances};

fn main() {
    run_for_files(2024, 1, "Part 1", &["sample", "input1"], get_sum_of_distances);
    run_for_files(2024, 1, "Part 2", &["sample", "input1"], get_similarity_score);
}
