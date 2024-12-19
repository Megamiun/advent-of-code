extern crate aoc;

use aoc::y2024::d18::ram_run::{find_min_blocker, find_min_steps_after};
use aoc::run_for_files;
use aoc::y2024::d18::ram_run_dfs::find_min_blocker_dfs;

fn main() {
    run_for_files(2024, 18, "pt1", &["sample"], |lines| find_min_steps_after(lines, 7, 12));
    run_for_files(2024, 18, "pt1", &["input"], |lines| find_min_steps_after(lines, 71, 1024));
    run_for_files(2024, 18, "pt2-priority-queue", &["sample"], |lines| find_min_blocker(lines, 7));
    run_for_files(2024, 18, "pt2-priority-queue", &["input"], |lines| find_min_blocker(lines, 71));
    run_for_files(2024, 18, "pt2-dfs", &["sample"], |lines| find_min_blocker_dfs(lines, 7));
    run_for_files(2024, 18, "pt2-dfs", &["input"], |lines| find_min_blocker_dfs(lines, 71));
}
