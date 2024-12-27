mod ceres_search_collecting;
mod ceres_search_cross_mas;
mod ceres_search_simple;

#[cfg(test)]
mod tests {
    use crate::y2024::d04::ceres_search_collecting::find_all_xmas_collecting;
    use crate::y2024::d04::ceres_search_cross_mas::find_all_cross_mas;
    use crate::y2024::d04::ceres_search_simple::find_all_xmas_simple;
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 18)]
    #[case::input("input", 2358)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 4, "Part 1", file, find_all_xmas_simple);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 18)]
    #[case::input("input", 2358)]
    fn part_1_collecting(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 4, "Part 1", file, find_all_xmas_collecting);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 9)]
    #[case::input("input", 1737)]
    fn part_2_cross_mas(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 4, "Part 2", file, find_all_cross_mas);
        assert_aoc!(result, expected)
    }
}
