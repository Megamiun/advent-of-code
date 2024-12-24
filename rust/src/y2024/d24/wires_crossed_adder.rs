use crate::y2024::d24::parse::{parse, EdgeMap, Operator};
use itertools::Itertools;
use std::collections::HashMap;

pub fn get_swappable_node_pairs_adder(groups: &[&[String]; 2], limit: usize) -> String {
    let (_, edges) = parse(groups);

    let zero_carry = get_result_for(&edges, "x00", "y00", "AND").unwrap();

    get_swaps(1, zero_carry, limit, &edges).unwrap()
        .iter().sorted().join(",")
}

fn get_swaps(level: usize, carry: &str, mistakes_left: usize, edges: &EdgeMap) -> Option<Vec<String>> {
    if edges.len() < 5 {
        return Some(vec![])
    }

    let level_str = pad_left(level);
    let x_level = &format!("x{level_str}");
    let y_level = &format!("y{level_str}");
    let z_level = &format!("z{level_str}");

    let z_result = get_inputs_for(edges, z_level, "XOR");
    if z_result.is_none() || !z_result.unwrap().contains(&carry) {
        let (carry_xor, _) = get_op_for(edges, carry, "XOR")?;
        return exchange(level, carry, mistakes_left, [z_level, carry_xor], edges)
    }

    let z_result = z_result.unwrap();

    let xy_xor = get_result_for(edges, x_level, y_level, "XOR").unwrap();
    let xy_and = get_result_for(edges, x_level, y_level, "AND").unwrap();

    if !z_result.contains(&xy_xor) {
        let non_carry = if carry == z_result[0] { z_result[1] } else { z_result[0] };
        return exchange(level, carry, mistakes_left, [xy_xor, non_carry], edges)
    }

    let z_xor = get_result_for(edges, carry, xy_xor, "XOR").unwrap();
    let z_and = get_result_for(edges, carry, xy_xor, "AND").unwrap();

    let new_carry = get_result_for(edges, xy_and, z_and, "OR").unwrap();

    let used = [xy_xor, xy_and, z_xor, z_and, new_carry];
    let used = Vec::from_iter(used.iter());
    let new_edges =
        HashMap::from_iter(edges.iter().filter(|(result, _)| !used.contains(result)).map(|s| (*s.0, *s.1)));

    get_swaps(level + 1, new_carry, mistakes_left, &new_edges)
}

fn exchange(level: usize, carry: &str, mistakes_left: usize, [lhs, rhs]: [&str; 2], edges: &EdgeMap) -> Option<Vec<String>> {
    if mistakes_left == 0 {
        return None
    }

    let mut new_edges = edges.clone();
    
    let lhs_result = edges[lhs];
    new_edges.insert(lhs, edges[rhs]);
    new_edges.insert(rhs, lhs_result);

    let mut mistakes = get_swaps(level, carry, mistakes_left - 1, &new_edges)?;
    mistakes.push(lhs.to_string());
    mistakes.push(rhs.to_string());
    Some(mistakes)
}

fn get_op_for<'a>(edges: &'a EdgeMap, input: &str, op: &str) -> Option<(&'a str, Operator<'a>)> {
    edges.iter()
        .filter(|(_, (edge_op, from))| from.contains(&input) && *edge_op == op)
        .map(|(result, inst)| (*result, *inst))
        .nth(0)
}

fn get_result_for<'a>(edges: &'a EdgeMap, lhs: &str, rhs: &str, op: &str) -> Option<&'a str> {
    let mut expected = [lhs, rhs];
    expected.sort();

    edges.iter()
        .filter(|(_, (edge_op, from))| *from == expected && *edge_op == op)
        .map(|(result, _)| *result)
        .nth(0)
}

fn get_inputs_for<'a>(edges: &'a EdgeMap, result: &str, op: &str) -> Option<[&'a str; 2]> {
    edges.iter()
        .filter(|(edge_result, (edge_op, _))| **edge_result == result && *edge_op == op)
        .map(|(_, (_, from))| *from)
        .nth(0)
}

fn pad_left(num: usize) -> String {
    format!("{:02}", num)
}
