mod race_condition;

#[cfg(test)]
mod tests {
    use crate::y2024::d20::race_condition::find_all_shortcuts_that_save_at_least;
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 1, 44)]
    #[case::input("input", 100, 1321)]
    fn part_1(#[case] file: &str, #[case] min_saved: usize, #[case] expected: usize) {
        let result = run_for_file(2024, 20, "Part 1", file, |lines| find_all_shortcuts_that_save_at_least(lines, 2, min_saved));
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 50, 285)]
    #[case::input("input", 100, 971737)]
    fn part_2(#[case] file: &str, #[case] min_saved: usize, #[case] expected: usize) {
        let result = run_for_file(2024, 20, "Part 2", file, |lines| find_all_shortcuts_that_save_at_least(lines, 20, min_saved));
        assert_aoc!(result, expected)
    }
}
