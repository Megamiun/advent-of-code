mod trebutchet;

#[cfg(test)]
mod tests {
    use crate::y2023::d01::trebutchet::{get_calibration_value_for, get_calibration_value_for_written};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 142)]
    #[case::input("input", 54697)]
    fn part_1(#[case] file: &str, #[case] expected: u32) {
        let result = run_for_file(2023, 1, "Part 1", file, get_calibration_value_for);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample-2", 281)]
    #[case::input("input", 54885)]
    fn part_2(#[case] file: &str, #[case] expected: u32) {
        let result = run_for_file(2023, 1, "Part 2", file, get_calibration_value_for_written);
        assert_aoc!(result, expected)
    }
}
