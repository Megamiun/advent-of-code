extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d04::ceres_search::find_all_xmas;

fn main() {
    run_for_files(2024, 4, &["sample-small", "sample", "input"], &find_all_xmas);
}
