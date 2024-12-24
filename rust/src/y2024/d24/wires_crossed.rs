use std::cmp::Reverse;
use itertools::Itertools;
use std::collections::HashMap;

pub fn define_z_output(groups: &[&[String]]) -> usize {
    let mut nodes = groups[0].iter()
        .map(|line| line.splitn(2, ": ").collect_vec())
        .map(|tokens| (tokens[0], tokens[1] == "1"))
        .collect::<HashMap<_, _>>();
    
    let edges  = groups[1].iter()
        .map(|line| line.splitn(5, " ").collect_vec())
        .map(|tokens| (tokens[4], [tokens[0], tokens[1], tokens[2] ]))
        .collect::<HashMap<_, _>>();
    
    let mut z_values = 0;
    
    edges.iter()
        .filter(|(key, _)| key.starts_with("z"))
        .sorted_by_key(|(key, _)| Reverse(*key))
        .map(|(key, _)| get_bit(&mut nodes, &edges, key))
        .for_each(|value| {
            z_values <<= 1;
            if value {
                z_values += 1
            }
        });
    
    z_values
}

fn get_bit<'a>(nodes: &mut HashMap<&'a str, bool>, edges: &HashMap<&str, [&'a str; 3]>, key: &'a str) -> bool {
    if let Some(value) = nodes.get(key) {
        return *value
    }
    
    let edge = edges[key];
    
    let result = match edge[1] { 
        "AND" => get_bit(nodes, edges, edge[0]) && get_bit(nodes, edges, edge[2]),
        "OR" => get_bit(nodes, edges, edge[0]) || get_bit(nodes, edges, edge[2]),
        _ => get_bit(nodes, edges, edge[0]) != get_bit(nodes, edges, edge[2])
    };
    
    *nodes.entry(key).or_insert(result)
}
