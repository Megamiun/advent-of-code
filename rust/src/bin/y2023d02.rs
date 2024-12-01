extern crate aoc;

use aoc::y2023::d02::cube_conumdrum::sum_minimum_power;
use aoc::y2023::d02::cube_conumdrum::sum_valid;
use aoc::{run_for_files, run_for_files_with_postfix};

fn main() {
    run_for_files_with_postfix(2023, 2, &["sample1", "input"], "limits", &sum_valid);
    run_for_files(2023, 2, &["sample1", "input"], &sum_minimum_power);
}
