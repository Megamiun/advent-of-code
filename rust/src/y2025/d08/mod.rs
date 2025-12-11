mod playground;

#[cfg(test)]
mod tests {
    use crate::y2025::d08::playground::{get_last_pair_x_product, get_sum_of_group_sizes};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 10, 40)]
    #[case::input("input", 1000, 83520)]
    fn part_1(#[case] file: &str, #[case] connections: usize, #[case] expected: usize) {
        let result = run_for_file(2025, 8, "Part 1", file, |lines| get_sum_of_group_sizes(lines, connections));
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 25272)]
    #[case::input("input", 1131823407)]
    fn part_2(#[case] file: &str, #[case] expected: u64) {
        let result = run_for_file(2025, 8, "Part 2", file, get_last_pair_x_product);
        assert_aoc!(result, expected)
    }
}
