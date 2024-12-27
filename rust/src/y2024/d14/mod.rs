mod restroom_redoubt;

#[cfg(test)]
mod tests {
    use crate::util::coordinates::Index2D;
    use crate::y2024::d14::restroom_redoubt::{get_safety_score, get_similar_to_tree};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 11, 7, 12)]
    #[case::input("input", 101, 103, 228457125)]
    fn part_1(#[case] file: &str, #[case] width: usize, #[case] height: usize, #[case] expected: usize) {
        let result = run_for_file(2024, 14, "Part 1", file, |group| {
            get_safety_score(group, &Index2D(width, height))
        });
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::input("input", 6493)]
    fn part_2(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 14, "Part 2", file, get_similar_to_tree);
        assert_aoc!(result, expected)
    }
}
