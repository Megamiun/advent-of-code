mod historian_hysteria;

#[cfg(test)]
mod tests {
    use crate::y2024::d01::historian_hysteria::{get_similarity_score, get_sum_of_distances};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 11)]
    #[case::input("input", 2742123)]
    fn part_1(#[case] file: &str, #[case] expected: i32) {
        let result = run_for_file(2024, 1, "Part 1", file, get_sum_of_distances);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", 31)]
    #[case::input("input", 21328497)]
    fn part_2(#[case] file: &str, #[case] expected: i32) {
        let result = run_for_file(2024, 1, "Part 2", file, get_similarity_score);
        assert_aoc!(result, expected)
    }
}
