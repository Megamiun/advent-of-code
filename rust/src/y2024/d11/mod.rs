mod plutonian_pebbles;

#[cfg(test)]
mod tests {
    use crate::y2024::d11::plutonian_pebbles::stones_after_steps;
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 55312)]
    #[case::input("input", 194482)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 11, "Part 1", file, |lines| stones_after_steps(lines, 25));
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::input("input", 232454623677743)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 11, "Part 2", file, |lines| stones_after_steps(lines, 75));
        assert_aoc!(result, expected)
    }
}
