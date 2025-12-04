mod printing_department;

#[cfg(test)]
mod tests {
    use crate::y2025::d04::printing_department::{get_accessible_rolls, get_all_accessible_rolls};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 13)]
    #[case::input("input", 1480)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2025, 4, "Part 1", file, get_accessible_rolls);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 43)]
    #[case::input("input", 8899)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2025, 4, "Part 2", file, get_all_accessible_rolls);
        assert_aoc!(result, expected)
    }
}
