use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::hash::BuildHasher;

pub fn get_lans_with_t(lines: &[String]) -> usize {
    let graph = get_graph(lines);

    graph
        .iter()
        .filter(|(node, _)| node.starts_with('t'))
        .flat_map(|(node, adjs)| adjs.iter().map(move |adj| [node, adj]))
        .flat_map(|[node_t, node_o]| 
            graph[node_t].iter().flat_map(|node_key| 
                graph[node_o].iter()
                    .filter(|node_third| node_key.clone() == **node_third)
                    .map(|node_third| [node_t.clone(), node_o.clone(), node_third.clone()])))
        .update(|s| s.sort())
        .unique()
        .count()
}

pub fn get_biggest_lan(lines: &[String]) -> String {
    let graph = get_graph(lines);
    let all_keys = graph.keys().collect::<Vec<_>>();
    
    get_biggest(&[], &all_keys, &graph).iter().sorted().copied().join(",")
}

fn get_biggest<'a>(group: &[&'a String], open: &[&'a String], graph: &HashMap<String, Vec<String>, impl BuildHasher>) -> Vec<&'a String> {
    let mut candidates = open.iter()
        .filter(|node| group.iter().all(|adj| graph[*adj].contains(*node)))
        .copied()
        .collect::<Vec<_>>();

    if candidates.is_empty() {
        return group.to_vec()
    }
    
    let mut all_maxes = Vec::<Vec<&'a String>>::with_capacity(candidates.len());
    
    while !candidates.is_empty() {
        let curr = candidates.pop().unwrap();
        let curr_max = get_biggest(
            &group.iter().copied().chain([curr].iter().copied()).collect::<Vec<_>>(),
            &candidates.iter().filter(|&s| graph[curr].contains(s)).copied().collect::<Vec<_>>(),
            graph);
        
        all_maxes.push(curr_max)
    }
    
    all_maxes.iter().max_by_key(|s| s.len()).unwrap().clone()
}

fn get_graph(lines: &[String]) -> HashMap<String, Vec<String>, impl BuildHasher> {
    let mut map = FxHashMap::<String, Vec<String>>::default();

    lines
        .iter()
        .filter_map(|line| line.split_once("-"))
        .for_each(|(left, right)| {
            map.entry(left.to_string()).or_insert_with(|| vec![]).push(right.to_string());
            map.entry(right.to_string()).or_insert_with(|| vec![]).push(left.to_string());
        });

    map
}
