extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d04::ceres_search_cross_mas::find_all_cross_mas;
use aoc::y2024::d04::ceres_search_simple::find_all_xmas_simple;

fn main() {
    // run_for_files(2024, 4, &["sample-small", "sample", "input"], &find_all_xmas_collecting);
    run_for_files(2024, 4, &["sample-small", "sample", "input"], &find_all_xmas_simple);
    run_for_files(2024, 4, &["sample-small", "sample", "input"], &find_all_cross_mas);
}
