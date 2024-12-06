use crate::util::{Diff, Index2D};
use rustc_hash::{FxHashSet};
use std::collections::{HashMap, HashSet};
use std::hash::BuildHasher;
use std::sync::LazyLock;

#[derive(PartialEq, Eq, Hash)]
struct Direction {
    dir: Diff,
    next: fn() -> &'static Direction,
}

impl Direction {
    const N: &'static Direction = &Direction { dir: Diff(0, -1), next: || Direction::E };
    const E: &'static Direction = &Direction { dir: Diff(1, 0), next: || Direction::S };
    const S: &'static Direction = &Direction { dir: Diff(0, 1), next: || Direction::W };
    const W: &'static Direction = &Direction { dir: Diff(-1, 0), next: || Direction::N };

    const VALUES: LazyLock<HashMap<char, &'static Direction>> = LazyLock::new(|| {
        HashMap::from([('^', Direction::N), ('>', Direction::E), ('V', Direction::S), ('<', Direction::W) ])
    });

    fn from(char: char) -> &'static Direction {
        Self::VALUES.get(&char).unwrap()
    }
}

pub fn get_visited_count(lines: &[String]) -> usize {
    let maze = as_maze(lines);

    let (guard_pos, guard_dir) = find_initial_guard_position(&maze);

    std::iter::successors(
        Option::from((guard_pos, guard_dir)),
        |(new_pos, new_dir)| visit_next(&maze, new_pos, new_dir),
    )
    .map(|(position, _)| position)
    .collect::<HashSet<_>>()
    .len()
}

pub fn get_loops_after_obstacle(lines: &[String]) -> usize {
    let maze = as_maze(lines);
    let (guard_pos, guard_dir) = find_initial_guard_position(&maze);
    let possible_obstacles = find_empty_positions(&maze);

    possible_obstacles.iter()
        .filter(|&&obstacle| has_loop(&maze, obstacle, guard_pos, guard_dir))
        .count()
}

fn has_loop(maze: &Vec<Vec<char>>, obstacle: Index2D, guard_pos: Index2D, guard_dir: &'static Direction) -> bool {
    let mut visited = create_hash_set_for(maze);
    let mut curr = (guard_pos, guard_dir);

    loop {
        if visited.contains(&curr) {
            return true
        }
        visited.insert(curr);

        let (position, direction) = curr;
        if position.add(direction.dir).is_some_and(|next| next == obstacle) {
            curr = (position, (direction.next)());
            continue
        }

        match visit_next(&maze, &position, direction) {
            Some(next) => curr = next,
            _ => return false
        };
    }
}

fn find_empty_positions(maze: &Vec<Vec<char>>) -> Vec<Index2D> {
    (0..maze.len()).flat_map(|y| {
        (0..maze[y].len())
            .filter(|&x| maze[y][x] == '.')
            .map(|x| Index2D(x, y))
            .collect::<Vec<_>>()
    }).collect()
}

fn find_initial_guard_position(maze: &Vec<Vec<char>>) -> (Index2D, &'static Direction) {
    (0..maze.len()).flat_map(|y| {
        (0..maze[y].len())
            .filter(|&x| !['.', '#'].contains(&maze[y][x]))
            .map(|x| (Index2D(x, y), Direction::from(maze[y][x])))
            .nth(0)
    })
    .nth(0)
    .unwrap()
}

fn visit_next(
    maze: &Vec<Vec<char>>,
    pos: &Index2D,
    direction: &'static Direction,
) -> Option<(Index2D, &'static Direction)> {
    let Index2D(x, y) = pos.add(direction.dir)?;
    let new_char = maze.get(y)?.get(x)?;

    match new_char {
        '#' => Option::from((pos.clone(), (direction.next)())),
        _ => Option::from((Index2D(x, y), direction)),
    }
}

fn as_maze(lines: &[String]) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|line| line.chars().map(|char| char).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn create_hash_set_for(maze: &Vec<Vec<char>>) -> HashSet<(Index2D, &Direction), impl BuildHasher> {
    let capacity = maze.len().pow(2);
    // HashSet::<(Index2D, &Direction)>::with_capacity(capacity)
    FxHashSet::<(Index2D, &Direction)>::with_capacity_and_hasher(capacity, Default::default())
}
