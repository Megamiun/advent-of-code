mod disk_fragmenter;
mod disk_fragmenter_linked_list;
mod helper;

#[cfg(test)]
mod tests {
    use crate::y2024::d09::disk_fragmenter::{fragment, reorder};
    use crate::y2024::d09::disk_fragmenter_linked_list::reorder_linked_list;
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 1928)]
    #[case::input("input", 6349606724455)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 9, "Part 1", file, fragment);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 2858)]
    #[case::input("input", 6376648986651)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 9, "Part 2", file, reorder);
        assert_aoc!(result, expected)
    }
    
    #[rstest]
    #[ignore]
    #[case::sample("sample", 2858)]
    #[ignore]
    #[case::input("input", 6376648986651)]
    fn part_2_linked_list(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 9, "Part 2", file, reorder_linked_list);
        assert_aoc!(result, expected)
    }
}
