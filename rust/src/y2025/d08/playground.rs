use crate::util::collections::key_indexed::keyable::Keyable;
use crate::util::parse_num::parse_u64;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::cmp::{max, min};

type XYZ = (u64, u64, u64);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Ord, PartialOrd)]
struct Entry<'a>(&'a XYZ, &'a XYZ);

impl<'a> Keyable for Entry<'a> {
    type Key = u64;
    fn get_key(&self) -> Self::Key {
        (self.0.0 - self.1.0).pow(2) + (self.0.1 - self.1.1).pow(2) + (self.0.2 - self.1.2).pow(2)
    }
}

#[allow(dead_code)]
pub fn get_sum_of_group_sizes(lines: &[String], connections: usize) -> usize {
    let positions = get_positions(lines);

    // TODO Check later why KeyPriorityQueue is so much slower
    let mut queue = Vec::<Entry>::new();
    let mut clusters = FxHashMap::<XYZ, usize>::default();

    for (lhs_index, lhs_pos) in positions.iter().enumerate() {
        clusters.insert(*lhs_pos, lhs_index);
        for rhs_pos in positions.iter().dropping(lhs_index + 1) {
            queue.push(Entry(lhs_pos, rhs_pos))
        }
    }

    queue.sort_by_cached_key(Entry::get_key);

    for Entry(lhs, rhs) in queue.iter().take(connections) {
        let lhs_cluster = clusters.get(lhs).unwrap();
        let rhs_cluster = clusters.get(rhs).unwrap();

        if lhs_cluster == rhs_cluster {
            continue
        }

        let min = *min(lhs_cluster, rhs_cluster);
        let max = *max(lhs_cluster, rhs_cluster);

        clusters.iter_mut()
            .filter(|(_, v)| **v == max)
            .for_each(|(_, v)| *v = min);
    }

    clusters.values().counts_by(|i| i)
        .values()
        .sorted().rev()
        .take(3)
        .fold(1, |acc, curr| acc * curr)
}

#[allow(dead_code)]
pub fn get_last_pair_x_product(lines: &[String]) -> u64 {
    let positions = get_positions(lines);

    // TODO Check later why KeyPriorityQueue is so much slower
    let mut queue = Vec::<Entry>::new();
    let mut clusters = FxHashMap::<XYZ, usize>::default();

    for (lhs_index, lhs_pos) in positions.iter().enumerate() {
        clusters.insert(*lhs_pos, lhs_index);
        for rhs_pos in positions.iter().dropping(lhs_index + 1) {
            queue.push(Entry(lhs_pos, rhs_pos))
        }
    }

    queue.sort_by_cached_key(Entry::get_key);

    for Entry(lhs, rhs) in queue.iter() {
        let lhs_cluster = clusters.get(lhs).unwrap();
        let rhs_cluster = clusters.get(rhs).unwrap();

        if lhs_cluster == rhs_cluster {
            continue
        }

        let min = *min(lhs_cluster, rhs_cluster);
        let max = *max(lhs_cluster, rhs_cluster);

        clusters.iter_mut()
            .filter(|(_, v)| **v == max)
            .for_each(|(_, v)| *v = min);

        if min == 0 && clusters.iter().all(|(_, v)| *v == 0) {
            return lhs.0 * rhs.0;
        }
    }

    panic!("No solution found")
}

fn get_positions(lines: &[String]) -> Vec<XYZ> {
    lines.iter()
        .map(|line| line.split(",").map(|num| parse_u64(num)).collect_tuple().unwrap())
        .collect_vec()
}
