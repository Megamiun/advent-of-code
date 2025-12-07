mod laboratories;

#[cfg(test)]
mod tests {
    use crate::y2025::d07::laboratories::{get_possibility_count, get_hit_splitters};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 21)]
    #[case::input("input", 1651)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2025, 7, "Part 1", file, get_hit_splitters);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 40)]
    #[case::input("input", 108924003331749)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2025, 7, "Part 2", file, get_possibility_count);
        assert_aoc!(result, expected)
    }
}
