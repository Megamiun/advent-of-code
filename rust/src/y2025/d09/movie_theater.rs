use crate::util::coordinates::Index2D;
use crate::util::parse_num::parse_usize;
use itertools::Itertools;

#[allow(dead_code)]
pub fn get_biggest_rectangle(lines: &[String]) -> usize {
    let positions = get_positions(lines);

    positions.iter().enumerate().flat_map(|(lhs_index, lhs)|
        positions.iter().dropping(lhs_index).map(|rhs|
            (lhs.0.abs_diff(rhs.0) + 1) * (lhs.1.abs_diff(rhs.1) + 1)
        )
    ).max().unwrap()
}

fn get_positions(lines: &[String]) -> Vec<Index2D> {
    lines.iter()
        .map(|line| line.split(",").map(|num| parse_usize(num)).collect_tuple().unwrap())
        .map(|(x, y)| Index2D(x, y))
        .collect_vec()
}
