mod factory;

#[cfg(test)]
mod tests {
    use crate::y2025::d10::factory::get_minimum_presses;
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 7)]
    #[case::input("input", 538)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2025, 10, "Part 1", file, get_minimum_presses);
        assert_aoc!(result, expected)
    }
}
