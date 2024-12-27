mod claw_contraption;

#[cfg(test)]
mod tests {
    use crate::y2024::d13::claw_contraption::{calculate, calculate_with_error};
    use crate::{assert_aoc, run_for_group};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 480)]
    #[case::input("input", 31065)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_group(2024, 13, "Part 1", file, calculate);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::input("input", 93866170395343)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_group(2024, 13, "Part 2", file, calculate_with_error);
        assert_aoc!(result, expected)
    }
}
