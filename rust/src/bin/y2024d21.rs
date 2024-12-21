extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d21::keypad_conundrum::get_sum_of_complexity;

fn main() {
    run_for_files(2024, 21, "Part 1", &["sample", "input"], get_sum_of_complexity);
}
