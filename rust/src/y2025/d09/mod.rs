mod movie_theater;

#[cfg(test)]
mod tests {
    use crate::y2025::d09::movie_theater::get_biggest_rectangle;
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 50)]
    #[case::input("input", 4777967538)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2025, 9, "Part 1", file, get_biggest_rectangle);
        assert_aoc!(result, expected)
    }

    // #[rstest]
    // #[case::sample("sample", 24)]
    // // #[case::input("input", 4777967538)]
    // fn part_2(#[case] file: &str, #[case] expected: usize) {
    //     let result = run_for_file(2025, 9, "Part 2", file, get_biggest_rectangle_inside);
    //     assert_aoc!(result, expected)
    // }
}
