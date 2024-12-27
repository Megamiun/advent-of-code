mod wires_crossed;
mod wires_crossed_adder;
mod parse;

#[cfg(test)]
mod tests {
    use crate::y2024::d24::wires_crossed::define_z_output;
    use crate::y2024::d24::wires_crossed_adder::get_swappable_node_pairs_adder;
    use crate::{assert_aoc, run_for_n_group};
    use rstest::rstest;

    #[rstest]
    #[case::sample("sample", 2024)]
    #[case::input("input", 56939028423824)]
    fn part_1(#[case] file: &str, #[case] expected: usize) {
        let result = run_for_n_group(2024, 24, "Part 1", file, define_z_output);
        assert_aoc!(result, expected)
    }

    #[rstest]
    #[case::input("input", 4, "frn,gmq,vtj,wnf,wtt,z05,z21,z39")]
    fn part_2(#[case] file: &str, #[case] pairs: usize, #[case] expected: &str) {
        let result = run_for_n_group(2024, 24, "Part 2", file, |lines| get_swappable_node_pairs_adder(lines, pairs));
        assert_aoc!(result, expected)
    }
}
