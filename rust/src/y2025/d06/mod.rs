mod trash_compactor;

#[cfg(test)]
mod tests {
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;
    use crate::y2025::d06::trash_compactor::{get_sum_of_solutions, get_sum_of_solutions_columnar};

    #[rstest]
    #[case::sample("sample", 4277556)]
    #[case::input("input", 6209956042374)]
    fn part_1(#[case] file: &str, #[case] expected: u64) {
        let result = run_for_file(2025, 6, "Part 1", file, get_sum_of_solutions);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 3263827)]
    #[case::input("input", 12608160008022)]
    fn part_2(#[case] file: &str, #[case] expected: u64) {
        let result = run_for_file(2025, 6, "Part 2", file, get_sum_of_solutions_columnar);
        assert_aoc!(result, expected)
    }
}
