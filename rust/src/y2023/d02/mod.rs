mod cube_conumdrum;

#[cfg(test)]
mod tests {
    use crate::y2023::d02::cube_conumdrum::{sum_minimum_power, sum_valid};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 8)]
    #[case::input("input", 2256)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2023, 2, "Part 1", file, |lines| sum_valid(lines, &[12, 13, 14]));
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 2286)]
    #[case::input("input", 74229)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2023, 2, "Part 2", file, sum_minimum_power);
        assert_aoc!(result, expected)
    }
}
