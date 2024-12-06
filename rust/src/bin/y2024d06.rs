extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d06::gallivant_guard::get_visited_count;

fn main() {
    run_for_files(2024, 6, &["sample", "input"], &get_visited_count);
}
