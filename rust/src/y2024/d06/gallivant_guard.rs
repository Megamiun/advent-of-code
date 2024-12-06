use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;
use derive_more::Display;
use crate::util::{Diff, Index2D};

#[derive(PartialEq, Eq, Hash, Display, Debug)]
#[display("({})", dir)]
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

    possible_obstacles.iter().filter(|Index2D(x, y)| {
        let mut copy: Vec<Vec<char>> = maze.clone();
        copy[*y][*x] = '#';

        has_loop(&copy, guard_pos, guard_dir)
    }).count()
}

fn has_loop(maze: &Vec<Vec<char>>, guard_pos: Index2D, guard_dir: &'static Direction) -> bool {
    let mut visited = HashSet::<(Index2D, &Direction)>::new();
    let mut has_cycle = false;

    std::iter::successors(
        Option::from((guard_pos, guard_dir)),
        |(new_pos, new_dir)| {
            let next_maybe = visit_next(&maze, new_pos, new_dir);
            let next = next_maybe?;

            if visited.contains(&next) {
                has_cycle = true;
                return None
            }

            visited.insert(next);
            next_maybe
        }
    ).count();

    has_cycle
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
    let Index2D(x, y) = add(pos, &direction.dir)?;
    let new_char = maze.get(y)?.get(x)?;

    match new_char {
        '#' => visit_next(maze, pos, (direction.next)()),
        _ => Option::from((Index2D(x, y), direction)),
    }
}

fn as_maze(lines: &[String]) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|line| line.chars().map(|char| char).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn add(Index2D(x, y): &Index2D, Diff(x_diff, y_diff): &Diff) -> Option<Index2D> {
    let new_x = usize::try_from(*x as i32 + x_diff);
    let new_y = usize::try_from(*y as i32 + y_diff);

    match (new_x, new_y) {
        (Ok(new_x_val), Ok(new_y_val)) => Some(Index2D(new_x_val, new_y_val)),
        _ => None,
    }
}
