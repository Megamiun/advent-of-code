extern crate aoc;

use aoc::y2024::d25::code_chronicle::get_combos;
use aoc::run_for_groups;

fn main() {
    run_for_groups(2024, 25, "Merry Christmas!", &["sample", "input"], get_combos);
}
