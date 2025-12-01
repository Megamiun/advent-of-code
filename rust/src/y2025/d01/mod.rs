mod secret_entrance;

#[cfg(test)]
mod tests {
    use crate::y2025::d01::secret_entrance::{get_clicks_at_zero, get_dials_at_zero};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 3)]
    #[case::input("input", 1154)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2025, 1, "Part 1", file, get_dials_at_zero);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 6)]
    #[case::input("input", 6819)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2025, 1, "Part 2", file, get_clicks_at_zero);
        assert_aoc!(result, expected)
    }
}
