extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d09::disk_fragmenter::{fragment, reorder};
use aoc::y2024::d09::disk_fragmenter_linked_list::reorder_linked_list;

fn main() {
    run_for_files(2024, 9, "Part 1", &["sample", "input"], fragment);
    run_for_files(2024, 9, "Part 2 simple", &["sample", "input"], reorder);
    run_for_files(2024, 9, "Part 2 linked-list", &["sample", "input"], reorder_linked_list);
}
