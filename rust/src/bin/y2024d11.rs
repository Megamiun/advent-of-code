extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d11::plutonian_peebles::{after_25_steps, after_75_steps};

fn main() {
    run_for_files(2024, 11, &["sample", "input"], &after_25_steps);
    run_for_files(2024, 11, &["sample", "input"], &after_75_steps);
}
