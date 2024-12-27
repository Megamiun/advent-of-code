mod hoof_it;

#[cfg(test)]
mod tests {
    use crate::y2024::d10::hoof_it::{get_reachable_sum_for, get_trails_sum_for};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 1)]
    #[case::sample("sample2", 36)]
    #[case::input("input", 719)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 10, "Part 1", file, get_reachable_sum_for);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample2", 81)]
    #[case::input("input", 1530)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 10, "Part 2", file, get_trails_sum_for);
        assert_aoc!(result, expected)
    }
}
