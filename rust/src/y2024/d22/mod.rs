mod monkey_market;

#[cfg(test)]
mod tests {
    use crate::y2024::d22::monkey_market::{get_max_bananas_after_4_numbers, get_sum_of_secrets_after};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 37327623)]
    #[case::input("input", 12759339434)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 22, "Part 1", file, get_sum_of_secrets_after);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample2", 23)]
    #[case::input("input", 1405)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 22, "Part 2", file, get_max_bananas_after_4_numbers);
        assert_aoc!(result, expected)
    }
}
