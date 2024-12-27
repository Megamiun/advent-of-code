mod gallivant_guard;

#[cfg(test)]
mod tests {
    use crate::y2024::d06::gallivant_guard::{get_loops_after_obstacle, get_visited_count};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 41)]
    #[case::input("input", 5305)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 6, "Part 1", file, get_visited_count);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 6)]
    #[case::input("input", 2143)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 6, "Part 2", file, get_loops_after_obstacle);
        assert_aoc!(result, expected)
    }
}
