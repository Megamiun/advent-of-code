use crate::util::Index2D;
use std::collections::{HashMap, HashSet};

pub fn part1(lines: &[String]) -> usize {
    let anthem_positions = get_anthem_indices(lines);

    let (height, width) = (lines.len(), lines[0].len());

    anthem_positions
        .iter()
        .flat_map(|(_, positions)| get_antinodes(positions))
        .collect::<HashSet<_>>()
        .iter().filter(|Index2D(x, y)| *y < height && *x < width)
        .count()
}

pub fn part2(lines: &[String]) -> i64 {
    0
}

fn get_antinodes(positions: &Vec<Index2D>) -> Vec<Index2D> {
    let length = positions.len();
    (0..length).flat_map(|first_index| {
        (first_index + 1..length).flat_map(|second_index| {
            get_antinodes_for(positions[first_index], positions[second_index])
        }).collect::<Vec<_>>()
    }).collect()
}

fn get_antinodes_for(first: Index2D, second: Index2D) -> Vec<Index2D> {
    let distance = first.get_distance_to(second);

    [first.add(distance), second.sub(distance)].iter()
        .filter_map(|maybe| *maybe)
        .collect()
}

fn get_anthem_indices(map: &[String]) -> HashMap<char, Vec<Index2D>> {
    let mut grouped_indices = HashMap::<char, Vec<Index2D>>::new();

    let anthem_positions = (0..map.len()).flat_map(|y| {
        (0..map[y].len())
            .filter_map(|x| Some((find_char(map, Index2D(x, y))?, Index2D(x, y))))
            .filter(|(char, _)| *char != '.')
            .collect::<Vec<_>>()
    });

    anthem_positions.for_each(|(char, index)| {
        if !grouped_indices.contains_key(&char) {
            grouped_indices.insert(char, Vec::new());
        };

        grouped_indices.get_mut(&char).unwrap().push(index)
    });

    grouped_indices
}

fn find_char(map: &[String], coord: Index2D) -> Option<char> {
    map.get(coord.0)?.chars().nth(coord.1)
}
