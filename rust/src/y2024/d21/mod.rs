mod keypad_conundrum;

#[cfg(test)]
mod tests {
    use crate::y2024::d21::keypad_conundrum::get_sum_of_complexity;
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 126384)]
    #[case::input("input", 203734)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 21, "Part 1", file, |lines| get_sum_of_complexity(lines, 2));
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::input("input", 246810588779586)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 21, "Part 2", file, |lines| get_sum_of_complexity(lines, 25));
        assert_aoc!(result, expected)
    }
}
