extern crate aoc;

use aoc::y2024::d20::race_condition::find_all_shortcuts_that_save_at_least;
use aoc::run_for_files;

fn main() {
    run_for_files(2024, 20, "Part 1", &["sample"], |lines| find_all_shortcuts_that_save_at_least(lines, 2, 0));
    run_for_files(2024, 20, "Part 1", &["input"], |lines| find_all_shortcuts_that_save_at_least(lines, 2, 100));
    run_for_files(2024, 20, "Part 2", &["sample"], |lines| find_all_shortcuts_that_save_at_least(lines, 20, 50));
    run_for_files(2024, 20, "Part 2", &["input"], |lines| find_all_shortcuts_that_save_at_least(lines, 20, 100));
}
