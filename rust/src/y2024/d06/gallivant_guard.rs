use crate::util::Index2D;
use crate::y2024::util::direction::Direction;
use crate::y2024::util::direction::Direction::{Down, Left, Right, Up};
use rustc_hash::FxHashSet;
use std::collections::{HashMap, HashSet};
use std::hash::BuildHasher;
use std::iter::successors;
use std::sync::LazyLock;

impl Direction {
    const CHAR_VALUES: LazyLock<HashMap<char, Direction>> = LazyLock::new(|| {
        HashMap::from([('^', Up), ('>', Right), ('V', Down), ('<', Left)])
    });

    fn from(char: char) -> Direction {
        Self::CHAR_VALUES[&char]
    }
    
    fn next(&self) -> Direction {
        self.get_clockwise()
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

fn get_unique_guard_positions(maze: &Vec<Vec<char>>, guard_pos: Index2D, guard_dir: Direction) -> HashSet<Index2D> {
    successors(
        Option::from((guard_pos, guard_dir)),
        |(new_pos, new_dir)| visit_next(&maze, new_pos, *new_dir),
    )
        .map(|(position, _)| position)
        .collect::<HashSet<_>>()
}

fn causes_loop(maze: &Vec<Vec<char>>, obstacle: Index2D, guard_pos: Index2D, guard_dir: Direction) -> bool {
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

fn find_initial_guard_position(maze: &Vec<Vec<char>>) -> (Index2D, Direction) {
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
    direction: Direction,
    obstacle: &Index2D,
) -> Option<(Index2D, Direction)> {
    let next = (pos + direction.get_dir())?;

    if *obstacle == next {
        return Option::from((*pos, direction.next()));
    }

    match maze.get(next.1)?.get(next.0)? {
        '#' => Option::from((*pos, direction.next())),
        _ => Option::from((next, direction)),
    }
}

fn visit_next(
    maze: &Vec<Vec<char>>,
    pos: &Index2D,
    direction: Direction,
) -> Option<(Index2D, Direction)> {
    let Index2D(x, y) = (pos + direction.get_dir())?;
    let new_char = maze.get(y)?.get(x)?;

    match new_char {
        '#' => Option::from((*pos, direction.next())),
        _ => Option::from((Index2D(x, y), direction)),
    }
}

fn as_maze(lines: &[String]) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|line| line.chars().map(|char| char).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn create_hash_set_for(maze: &Vec<Vec<char>>) -> HashSet<(Index2D, Direction), impl BuildHasher> {
    let capacity = maze.len().pow(2);
    // Default impl makes this slower (From 41ms to 60ms on my computer) for the complete flow
    // HashSet::<(Index2D, &Direction)>::with_capacity(capacity)
    FxHashSet::<(Index2D, Direction)>::with_capacity_and_hasher(capacity, Default::default())
}
