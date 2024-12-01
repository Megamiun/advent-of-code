extern crate aoc;

use aoc::run_for_files;
use aoc::y2023::d01::trebutchet::get_calibration_value_for;

fn main() {
    run_for_files(2023, 1, &["sample1", "sample2", "input"], &get_calibration_value_for);
}
