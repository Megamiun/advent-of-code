mod lobby;

#[cfg(test)]
mod tests {
    use crate::y2025::d03::lobby::get_sum_of_joltage;
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 357)]
    #[case::input("input", 17321)]
    fn part_1(#[case] file: &str, #[case] expected: u64) {
        let result = run_for_file(2025, 3, "Part 1", file, |file| get_sum_of_joltage(file, 2));
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 3121910778619)]
    #[case::input("input", 171989894144198)]
    fn part_2(#[case] file: &str, #[case] expected: u64) {
        let result = run_for_file(2025, 3, "Part 2", file, |file| get_sum_of_joltage(file, 12));
        assert_aoc!(result, expected)
    }
}
