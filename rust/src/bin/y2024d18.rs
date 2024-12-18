extern crate aoc;

use aoc::y2024::d18::ram_run::{find_min_blocker, find_min_steps_after};
use aoc::run_for_files;

fn main() {
    run_for_files(2024, 18, &["sample"], &|lines| find_min_steps_after(lines, 7, 12));
    run_for_files(2024, 18, &["input"], &|lines| find_min_steps_after(lines, 71, 1024));
    run_for_files(2024, 18, &["sample"], &|lines| find_min_blocker(lines, 7));
    run_for_files(2024, 18, &["input"], &|lines| find_min_blocker(lines, 71));
}
