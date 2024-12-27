mod chronospatial_computer;

#[cfg(test)]
mod tests {
    use crate::y2024::d17::chronospatial_computer::{execute, find};
    use crate::{assert_aoc, run_for_n_group};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", "4,6,3,5,6,3,5,2,1,0")]
    #[case::input("input", "1,5,3,0,2,5,2,5,3")]
    fn part_1(#[case] file: &str, #[case] expected: &str) {
        let result = run_for_n_group(2024, 17, "Part 1", file, execute);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 117440)]
    #[case::input("input", 108107566389757)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        // let result = run_for_n_group(2024, 17, "Part 2", file, find);
        // assert_aoc!(result, expected)
        panic!("Not impl");
    }
}
