mod linen_layout;

#[cfg(test)]
mod tests {
    use crate::y2024::d19::linen_layout::{get_possible_towels, get_towels_arrangements};
    use crate::{assert_aoc, run_for_n_group};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 6)]
    #[case::input("input", 360)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_n_group(2024, 19, "Part 1", file, get_possible_towels);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 16)]
    #[case::input("input", 577474410989846)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_n_group(2024, 19, "Part 2", file, get_towels_arrangements);
        assert_aoc!(result, expected)
    }
}
