use crate::y2024::d24::parse::{parse, EdgeMap};
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn define_z_output(groups: &[&[String]; 2]) -> usize {
    let (mut nodes, edges) = parse(groups);
    let z_bits = get_z_bits(&mut nodes, &edges);

    let mut z_values = 0;
    z_bits.iter().for_each(|value| {
        z_values <<= 1;
        if *value {
            z_values += 1
        }
    });

    z_values
}

#[allow(dead_code)]
fn get_z_bits<'a>(nodes: &mut HashMap<&'a str, bool>, edges: &'a EdgeMap) -> Vec<bool> {
    edges.iter()
        .filter(|(key, _)| key.starts_with("z"))
        .sorted_by_key(|(key, _)| Reverse(*key))
        .map(|(key, _)| get_bit(nodes, edges, key))
        .collect_vec()
}

fn get_bit<'a>(nodes: &mut HashMap<&'a str, bool>, edges: &'a EdgeMap, key: &'a str) -> bool {
    if let Some(value) = nodes.get(key) {
        return *value;
    }

    let edge = edges[key];
    let result = match edge.0 {
        "AND" => get_bit(nodes, edges, edge.1[0]) && get_bit(nodes, edges, edge.1[1]),
        "OR" => get_bit(nodes, edges, edge.1[0]) || get_bit(nodes, edges, edge.1[1]),
        _ => get_bit(nodes, edges, edge.1[0]) != get_bit(nodes, edges, edge.1[1])
    };

    *nodes.entry(key).or_insert(result)
}
