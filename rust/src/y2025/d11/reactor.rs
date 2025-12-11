use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashMap;

#[allow(dead_code)]
pub fn get_possible_paths(lines: &[String]) -> usize {
    let mut graph = FxHashMap::default();
    graph.extend(get_graph(lines));

    get_paths_to(&graph, "you", "out").unwrap_or(0)
}

#[allow(dead_code)]
pub fn get_possible_paths_with_dac_and_fft(lines: &[String]) -> usize {
    let graph = &mut FxHashMap::default();
    graph.extend(get_graph(lines));

    let dac_to_fft = get_paths_to(graph, "dac", "fft");
    match dac_to_fft {
        Some(dac_to_fft) => {
            let svr_to_dac = get_paths_to(graph, "svr", "dac").unwrap_or_else(|| panic!("No svr_to_dac found!"));
            let fft_to_out = get_paths_to(graph, "fft", "out").unwrap_or_else(|| panic!("No fft_to_out found!"));
            svr_to_dac * dac_to_fft * fft_to_out
        },
        None => {
            let svr_to_fft = get_paths_to(graph, "svr", "fft").unwrap_or_else(|| panic!("No svr_to_fft found!"));
            let fft_to_dac = get_paths_to(graph, "fft", "dac").unwrap_or_else(|| panic!("No fft_to_dac found!"));
            let dac_to_out = get_paths_to(graph, "dac", "out").unwrap_or_else(|| panic!("No dac_to_out found!"));
            svr_to_fft * fft_to_dac * dac_to_out
        }
    }
}

fn get_paths_to(graph: &FxHashMap<&str, Vec<&str>>, from: &str, to: &str) -> Option<usize> {
    let mut cache = FxHashMap::default();
    cache.insert(to, Some(1));

    seek_path(&graph, from, &mut cache)
}

fn seek_path<'a>(graph: &FxHashMap<&str, Vec<&'a str>>, from: &'a str, cache: &mut FxHashMap<&'a str, Option<usize>>) -> Option<usize> {
    if cache.contains_key(from) {
        return *cache.get(from).unwrap();
    }

    if !graph.contains_key(from) {
        cache.insert(from, None);
        return None
    }

    let result = graph[from].iter()
        .filter_map(|next| seek_path(graph, next, cache))
        .sum();

    let result = Some(result).filter(|&x| x != 0);
    cache.insert(from, result);
    result
}

fn get_graph(lines: &[String]) -> Vec<(&str, Vec<&str>)> {
    let node_regex = Regex::new(r"[a-z]*").unwrap();

    lines.iter().map(|line| {
        let mut capture = node_regex.find_iter(line);
        (
            capture.nth(0).unwrap().as_str(),
            capture.map(|m| m.as_str()).filter(|item| !item.is_empty()).collect_vec()
        )
    }).collect_vec()
}
