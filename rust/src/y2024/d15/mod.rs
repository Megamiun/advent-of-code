mod warehouse_woes;

#[cfg(test)]
mod tests {
    use crate::y2024::d15::warehouse_woes::{move_robot_single, move_robot_wide};
    use crate::{assert_aoc, run_for_n_group};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample-small", 2028)]
    #[case::sample("sample", 10092)]
    #[case::input("input", 1430536)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_n_group(2024, 15, "Part 1", file, move_robot_single);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 9021)]
    #[case::input("input", 1452348)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_n_group(2024, 15, "Part 2", file, move_robot_wide);
        assert_aoc!(result, expected)
    }
}
