use std::collections::HashSet;
use itertools::Itertools;
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

#[allow(dead_code)]
pub fn get_lans_with_t(lines: &[String]) -> usize {
    let graph = get_graph(lines);

    graph
        .iter()
        .filter(|(node, _)| node.starts_with('t'))
        .flat_map(|(node, adjs)| adjs.iter().map(move |adj| [node, adj]))
        .flat_map(|[node_t, node_o]|
            graph[node_t].iter().flat_map(|node_key|
                graph[node_o].iter()
                    .filter(|node_third| *node_key == **node_third)
                    .map(|node_third| [node_t, node_o, node_third])).collect_vec())
        .update(|s| s.sort())
        .unique().count()
}

#[allow(dead_code)]
pub fn get_biggest_lan(lines: &[String]) -> String {
    let graph = get_graph(lines);
    let all_keys = graph.keys().collect::<Vec<_>>();

    get_biggest(&[], &all_keys, &graph).iter().sorted().join(",")
}

fn get_biggest<'a>(group: &[&'a String], open: &[&'a String], graph: &FxHashMap<String, FxHashSet<String>>) -> Vec<&'a String> {
    if open.is_empty() {
        return group.to_vec()
    }
    
    open.iter().enumerate().map(|(index, &curr)| {
        let group_with_curr = group.iter().chain([curr].iter())
            .copied().collect_vec();

        let curr_intersection = open[index + 1..].iter()
            .filter(|&&s| graph[curr].contains(s))
            .copied().collect_vec();

        get_biggest(&group_with_curr, &curr_intersection, graph)
    }).max_by_key(|s| s.len()).unwrap()
}

fn get_graph(lines: &[String]) -> FxHashMap<String, FxHashSet<String>> {
    let mut map = FxHashMap::<String, FxHashSet<String>>::default();

    lines.iter()
        .filter_map(|line| line.split_once("-"))
        .for_each(|(left, right)| {
            map.entry(left.to_string()).or_insert_with(create_hash_set).insert(right.to_string());
            map.entry(right.to_string()).or_insert_with(create_hash_set).insert(left.to_string());
        });
    map
}

fn create_hash_set() -> HashSet<String, FxBuildHasher> {
    FxHashSet::with_capacity_and_hasher(13, Default::default())
}
