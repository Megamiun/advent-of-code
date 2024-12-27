mod resonant_collinearity;

#[cfg(test)]
mod tests {
    use crate::y2024::d08::resonant_collinearity::{get_antinodes_for_repeated_jumps, get_antinodes_for_single_jump};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 14)]
    #[case::input("input", 361)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 8, "Part 1", file, get_antinodes_for_single_jump);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 34)]
    #[case::input("input", 1249)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 8, "Part 2", file, get_antinodes_for_repeated_jumps);
        assert_aoc!(result, expected)
    }
}
