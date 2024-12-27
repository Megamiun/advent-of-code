mod bridge_repair;

#[cfg(test)]
mod tests {
    use crate::y2024::d07::bridge_repair::{with_basic_operators, with_extra_operators};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 3749)]
    #[case::input("input", 42283209483350)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 7, "Part 1", file, with_basic_operators);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 11387)]
    #[case::input("input", 1026766857276279)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 7, "Part 2", file, with_extra_operators);
        assert_aoc!(result, expected)
    }
}
