extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d04::ceres_search::find_all_xmas;
use aoc::y2024::d04::ceres_search_xmas::find_all_cross_mas;

fn main() {
    run_for_files(2024, 4, &["sample-small", "sample", "input"], &find_all_xmas);
    run_for_files(2024, 4, &["sample-small", "sample", "input"], &find_all_cross_mas);
}
