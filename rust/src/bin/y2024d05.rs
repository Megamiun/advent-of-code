extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d05::print_queue::{get_sum_of_correct_middle_points, get_sum_of_incorrect_middle_points};

fn main() {
    run_for_files(2024, 5, "pt1", &["sample", "input"], get_sum_of_correct_middle_points);
    run_for_files(2024, 5, "pt2", &["sample", "input"], get_sum_of_incorrect_middle_points);
}
