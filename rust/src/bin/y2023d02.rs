extern crate aoc;

use aoc::run_for_files;
use aoc::y2023::d02::cube_conumdrum::sum_minimum_power;
use aoc::y2023::d02::cube_conumdrum::sum_valid;

fn main() {
    run_for_files(2023, 2, "pt1", &["sample1", "input"], |lines| sum_valid(lines, &vec![12usize, 13usize, 14usize]));
    run_for_files(2023, 2, "pt2", &["sample1", "input"], sum_minimum_power);
}
