mod gift_shop;

#[cfg(test)]
mod tests {
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;
    use crate::y2025::d02::gift_shop::{get_numbers_with_repeated_digits, get_numbers_with_repeated_digits_boundless};

    #[rstest]
    #[case::sample("sample", 1227775554)]
    #[case::input("input", 38158151648)]
    fn part_1(#[case] file: &str, #[case] expected: u64) {
        let result = run_for_file(2025, 2, "Part 1", file, get_numbers_with_repeated_digits);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 4174379265)]
    #[case::input("input", 45283684555)]
    fn part_2(#[case] file: &str, #[case] expected: u64) {
        let result = run_for_file(2025, 2, "Part 2", file, get_numbers_with_repeated_digits_boundless);
        assert_aoc!(result, expected)
    }
}
