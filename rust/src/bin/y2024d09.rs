extern crate aoc;

use aoc::run_for_files;
use aoc::y2024::d09::disk_fragmenter::fragment;
use aoc::y2024::d09::disk_fragmenter_linked_list::reorder_linked_list;

fn main() {
    run_for_files(2024, 9, &["sample", "input"], &fragment);
    run_for_files(2024, 9, &["sample", "input"], &reorder_linked_list);
}
