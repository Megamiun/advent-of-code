mod parse;
mod ram_run;
mod ram_run_dfs;

#[cfg(test)]
mod tests {
    use crate::y2024::d18::ram_run::find_min_steps_after;
    use crate::y2024::d18::ram_run_dfs::find_min_blocker_dfs;
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 7, 12, 22)]
    #[case::input("input", 71, 1024, 272)]
    fn part_1(#[case] file: &str, #[case] dimensions: usize, #[case] bytes: usize, #[case] expected: usize) {
        let result = run_for_file(2024, 18, "Part 1", file, |lines| find_min_steps_after(lines, dimensions, bytes));
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 7, "6,1")]
    #[case::input("input", 71, "16,44")]
    fn part_2(#[case] file: &str, #[case] size: usize, #[case] expected: &str) {
        let result = run_for_file(2024, 18, "Part 2", file, |lines| find_min_blocker_dfs(lines, size));
        assert_aoc!(result, expected)
    }
}
