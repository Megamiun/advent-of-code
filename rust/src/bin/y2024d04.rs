extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d04::ceres_search_collecting::find_all_xmas_collecting;
use aoc::y2024::d04::ceres_search_cross_mas::find_all_cross_mas;
use aoc::y2024::d04::ceres_search_simple::find_all_xmas_simple;

fn main() {
    run_for_files(2024, 4, "Part 1", &["sample-small", "sample", "input"], find_all_xmas_simple);
    run_for_files(2024, 4, "Part 2 simple", &["sample-small", "sample", "input"], find_all_cross_mas);
    run_for_files(2024, 4, "Part 2 collecting", &["sample-small", "sample", "input"], find_all_xmas_collecting);
}
