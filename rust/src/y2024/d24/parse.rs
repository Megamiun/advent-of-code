use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;

pub type Operator<'a> = (&'a str, [&'a str; 2]);
pub type EdgeMap<'a> = HashMap<&'a str, Operator<'a>>;

pub fn parse<'a>([nodes, edges]: &'a [&[String]; 2]) -> (HashMap<&'a str, bool>, EdgeMap<'a>) {
    let nodes = nodes.iter()
        .map(|line| line.splitn(2, ": ").collect_vec())
        .map(|tokens| (tokens[0], tokens[1] == "1"))
        .collect::<HashMap<_, _>>();

    let edges = edges.iter()
        .map(|line| line.splitn(5, " ").collect_vec())
        .map(|tokens| (tokens[4], (tokens[1], [min(tokens[0], tokens[2]), max(tokens[0], tokens[2])])))
        .collect::<HashMap<_, _>>();
    (nodes, edges)
}
