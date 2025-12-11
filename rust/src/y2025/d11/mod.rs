mod reactor;

#[cfg(test)]
mod tests {
    use crate::y2025::d11::reactor::{get_possible_paths, get_possible_paths_with_dac_and_fft};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 5)]
    #[case::input("input", 428)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2025, 11, "Part 1", file, get_possible_paths);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample-2", 2)]
    #[case::input("input", 331468292364745)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2025, 11, "Part 2", file, get_possible_paths_with_dac_and_fft);
        assert_aoc!(result, expected)
    }
}
