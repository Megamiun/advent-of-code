mod garden_groups;

#[cfg(test)]
mod tests {
    use crate::y2024::d12::garden_groups::{find_price_by_perimeter, find_price_by_sides};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 1930)]
    #[case::input("input", 1450816)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 12, "Part 1", file, find_price_by_perimeter);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 1206)]
    #[case::input("input", 865662)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 12, "Part 2", file, find_price_by_sides);
        assert_aoc!(result, expected)
    }
}
