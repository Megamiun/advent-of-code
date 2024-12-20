extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d11::plutonian_pebbles::stones_after_steps;

fn main() {
    run_for_files(2024, 11, "Part 1", &["sample", "input"], |lines| stones_after_steps(lines, 25));
    run_for_files(2024, 11, "Part 2", &["sample", "input"], |lines| stones_after_steps(lines, 75));
}
