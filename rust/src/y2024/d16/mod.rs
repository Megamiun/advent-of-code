mod reindeer_maze;

#[cfg(test)]
mod tests {
    use crate::y2024::d16::reindeer_maze::{get_min_path, get_unique_best_spots};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 7036)]
    #[case::sample2("sample-2", 11048)]
    #[case::input("input", 98416)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 16, "Part 1", file, get_min_path);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 45)]
    #[case::sample2("sample-2", 64)]
    #[case::input("input", 471)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 16, "Part 2", file, get_unique_best_spots);
        assert_aoc!(result, expected)
    }
}
