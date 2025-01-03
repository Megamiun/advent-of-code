mod print_queue;

#[cfg(test)]
mod tests {
    use crate::y2024::d05::print_queue::{get_sum_of_correct_middle_points, get_sum_of_incorrect_middle_points};
    use crate::{assert_aoc, run_for_n_group};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 143)]
    #[case::input("input", 5639)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_n_group(2024, 5, "Part 1", file, get_sum_of_correct_middle_points);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 123)]
    #[case::input("input", 5273)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_n_group(2024, 5, "Part 2", file, get_sum_of_incorrect_middle_points);
        assert_aoc!(result, expected)
    }
}
