use crate::util::{Diff, Index2D};
use rustc_hash::FxHashSet;
use std::collections::{HashMap, HashSet};
use std::hash::BuildHasher;
use std::iter::successors;
use std::sync::LazyLock;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
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
        HashMap::from([('^', Direction::N), ('>', Direction::E), ('V', Direction::S), ('<', Direction::W)])
    });

    fn from(char: char) -> &'static Direction {
        Self::VALUES.get(&char).unwrap()
    }
}

pub fn get_visited_count(lines: &[String]) -> usize {
    let maze = as_maze(lines);

    let (guard_pos, guard_dir) = find_initial_guard_position(&maze);

    get_unique_guard_positions(&maze, guard_pos, guard_dir).len()
}

pub fn get_loops_after_obstacle(lines: &[String]) -> usize {
    let maze = as_maze(lines);
    let (guard_pos, guard_dir) = find_initial_guard_position(&maze);

    get_unique_guard_positions(&maze, guard_pos, guard_dir).iter()
        .filter(|&obstacle| causes_loop(&maze, *obstacle, guard_pos, guard_dir))
        .count()
}

fn get_unique_guard_positions(maze: &Vec<Vec<char>>, guard_pos: Index2D, guard_dir: &'static Direction) -> HashSet<Index2D> {
    successors(
        Option::from((guard_pos, guard_dir)),
        |(new_pos, new_dir)| visit_next(&maze, new_pos, new_dir),
    )
        .map(|(position, _)| position)
        .collect::<HashSet<_>>()
}

fn causes_loop(maze: &Vec<Vec<char>>, obstacle: Index2D, guard_pos: Index2D, guard_dir: &'static Direction) -> bool {
    let mut visited = create_hash_set_for(maze);
    let mut curr = (guard_pos, guard_dir);

    loop {
        let (position, direction) = curr;
        let maybe_next = visit_next_with_obstacle(maze, &position, direction, &obstacle);

        if let Some(next) = maybe_next {
            if next.1 != direction {
                if visited.contains(&curr) {
                    return true;
                }
                visited.insert(curr);
            }

            curr = next
        } else {
            return false
        };
    }
}

fn find_initial_guard_position(maze: &Vec<Vec<char>>) -> (Index2D, &'static Direction) {
    (0..maze.len()).flat_map(|y| {
        (0..maze[y].len())
            .filter(|&x| !['.', '#'].contains(&maze[y][x]))
            .map(|x| (Index2D(x, y), Direction::from(maze[y][x])))
            .nth(0)
    }).nth(0).unwrap()
}

fn visit_next_with_obstacle(
    maze: &Vec<Vec<char>>,
    pos: &Index2D,
    direction: &'static Direction,
    obstacle: &Index2D,
) -> Option<(Index2D, &'static Direction)> {
    let next = (pos + direction.dir)?;

    if *obstacle == next {
        return Option::from((pos.clone(), (direction.next)()));
    }

    match maze.get(next.1)?.get(next.0)? {
        '#' => Option::from((pos.clone(), (direction.next)())),
        _ => Option::from((next, direction)),
    }
}

fn visit_next(
    maze: &Vec<Vec<char>>,
    pos: &Index2D,
    direction: &'static Direction,
) -> Option<(Index2D, &'static Direction)> {
    let Index2D(x, y) = (pos + direction.dir)?;
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
    // Default impl makes this slower (From 41ms to 60ms on my computer) for the complete flow
    // HashSet::<(Index2D, &Direction)>::with_capacity(capacity)
    FxHashSet::<(Index2D, &Direction)>::with_capacity_and_hasher(capacity, Default::default())
}
