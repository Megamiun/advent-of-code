mod code_chronicle;

#[cfg(test)]
mod tests {
    use crate::y2024::d25::code_chronicle::get_combos;
    use crate::{assert_aoc, run_for_group};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 3)]
    #[case::input("input", 3107)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_group(2024, 25, "Part 1", file, get_combos);
        assert_aoc!(result, expected)
    }
}
