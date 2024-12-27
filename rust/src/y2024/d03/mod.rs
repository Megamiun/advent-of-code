mod mull_it_over;

#[cfg(test)]
mod tests {
    use crate::y2024::d03::mull_it_over::{sum_muls, sum_muls_in_do};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 161)]
    #[case::input("input", 185797128)]
    fn part_1(#[case] file: &str, #[case] expected: u32) {
        let result = run_for_file(2024, 3, "Part 1", file, sum_muls);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 48)]
    #[case::input("input", 89798695)]
    fn part_2(#[case] file: &str, #[case] expected: u32) {
        let result = run_for_file(2024, 3, "Part 2", file, sum_muls_in_do);
        assert_aoc!(result, expected)
    }
}
