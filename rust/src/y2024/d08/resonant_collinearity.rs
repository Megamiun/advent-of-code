use crate::util::coordinates::Index2D;
use crate::util::bounded::Bounded;
use std::collections::{HashMap, HashSet};
use std::iter::successors;

pub fn get_antinodes_for_single_jump(lines: &[String]) -> usize {
    let city = Bounded::from(lines);
    let anthem_positions = city.get_anthem_indices();

    anthem_positions
        .iter()
        .flat_map(|(_, positions)| city.get_antinodes(positions))
        // only counts segments with two or more antinodes
        .filter_map(|path| path.get(1).cloned())
        .collect::<HashSet<_>>()
        .len()
}

pub fn get_antinodes_for_repeated_jumps(lines: &[String]) -> usize {
    let city = Bounded::from(lines);
    let anthem_positions = city.get_anthem_indices();

    anthem_positions
        .iter()
        .flat_map(|(_, positions)| city.get_antinodes(positions))
        .flatten()
        .collect::<HashSet<_>>()
        .len()
}

impl Bounded<char> {
    fn get_antinodes(&self, positions: &Vec<Index2D>) -> Vec<Vec<Index2D>> {
        let length = positions.len();

        (0..length).flat_map(|i| {
            (i + 1..length).flat_map(|j| 
                self.get_antinode_lines(positions[i], positions[j])
            ).collect::<Vec<_>>()
        }).collect()
    }

    /// Return two segments, one coming from the first node, another from the second.
    /// This allows us to know if a node is responsible for at least one antinode later.
    fn get_antinode_lines(&self, first: Index2D, second: Index2D) -> Vec<Vec<Index2D>> {
        let distance = first - second;

        vec![
            self.get_within_bounds(first, |acc| acc + distance),
            self.get_within_bounds(second, |acc| acc - distance),
        ]
    }

    fn get_within_bounds(&self, first: Index2D, get_next: impl Fn(&Index2D) -> Option<Index2D>) -> Vec<Index2D> {
        successors(Some(first), get_next)
            .take_while(|item| self.is_within(item))
            .collect()
    }

    fn get_anthem_indices(&self) -> HashMap<char, Vec<Index2D>> {
        let anthem_positions = self.content.iter().enumerate().flat_map(|(y, line)| {
            line.iter().enumerate()
                .filter_map(|(x, char)| Some((char, Index2D(x, y))))
                .filter(|(&char, _)| char != '.')
                .collect::<Vec<_>>()
        });

        let mut grouped_indices = HashMap::<char, Vec<Index2D>>::new();
        anthem_positions.for_each(|(char, index)| {
            if !grouped_indices.contains_key(&char) {
                grouped_indices.insert(*char, Vec::new());
            };

            grouped_indices.get_mut(&char).unwrap().push(index)
        });

        grouped_indices
    }
}