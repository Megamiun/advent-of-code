mod red_nosed_reports;

#[cfg(test)]
mod tests {
    use crate::y2024::d02::red_nosed_reports::{count_safe, count_safe_with_tolerance};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 2)]
    #[case::input("input", 559)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 2, "Part 1", file, count_safe);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 4)]
    #[case::input("input", 601)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 2, "Part 2", file, count_safe_with_tolerance);
        assert_aoc!(result, expected)
    }
}
