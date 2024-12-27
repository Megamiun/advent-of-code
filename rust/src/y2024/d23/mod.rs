mod lan_party;

#[cfg(test)]
mod tests {
    use crate::y2024::d23::lan_party::{get_biggest_lan, get_lans_with_t};
    use crate::{assert_aoc, run_for_file};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 7)]
    #[case::input("input", 1163)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_file(2024, 23, "Part 1", file, get_lans_with_t);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::sample("sample", "co,de,ka,ta")]
    #[case::input("input", "bm,bo,ee,fo,gt,hv,jv,kd,md,mu,nm,wx,xh")]
    fn part_2(#[case] file: &str, #[case] expected: &str) {
        let result = run_for_file(2024, 23, "Part 2", file, get_biggest_lan);
        assert_aoc!(result, expected)
    }
}
