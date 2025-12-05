mod cafeteria;

#[cfg(test)]
mod tests {
    use crate::y2025::d05::cafeteria::{get_fresh_count, get_whole_fresh_ingredients};
    use crate::{assert_aoc, run_for_n_group};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 3)]
    #[case::input("input", 640)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_n_group(2025, 5, "Part 1", file, get_fresh_count);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 14)]
    #[case::input("input", 365804144481581)]
    fn part_2(#[case] file: &str, #[case] expected: u64) {
        let result = run_for_n_group(2025, 5, "Part 2", file, get_whole_fresh_ingredients);
        assert_aoc!(result, expected)
    }
}
