extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d14::restroom_redoubt::{get_safety_score, get_similar_to_tree};

fn main() {
    run_for_files(2024, 14, &["sample", "input"], &get_safety_score);
    run_for_files(2024, 14, &["input"], &get_similar_to_tree);
}
