use crate::y2024::d06::gallivant_guard::Direction::{E, N, S, W};
use std::collections::HashSet;
use std::panic::panic_any;

enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    const DIRECTIONS: &'static [(i32, i32)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];

    fn get_dir(&self) -> &(i32, i32) {
        match self {
            N => &Self::DIRECTIONS[0],
            E => &Self::DIRECTIONS[1],
            S => &Self::DIRECTIONS[2],
            W => &Self::DIRECTIONS[3],
        }
    }

    fn from(char: char) -> &'static Direction {
        match char {
            '^' => &N,
            '>' => &E,
            'V' => &S,
            '<' => &W,
            _ => panic_any(format!("Direction unknown: {char}")),
        }
    }

    fn next(&self) -> &'static Direction {
        match self {
            N => &E,
            E => &S,
            S => &W,
            W => &N,
        }
    }
}

pub fn get_visited_count(lines: &[String]) -> usize {
    let maze = as_maze(lines);

    let (guard_pos, guard_dir) = (0..maze.len())
        .flat_map(|y| {
            (0..maze[y].len())
                .filter(|&x| !['.', '#'].contains(&maze[y][x]))
                .map(|x| ((x, y), Direction::from(maze[y][x])))
                .nth(0)
        })
        .nth(0)
        .unwrap();

    std::iter::successors(
        Option::from((guard_pos, guard_dir)),
        |(new_pos, new_dir)| visit_next(&maze, new_pos, new_dir),
    )
    .map(|(position, _)| position)
    .collect::<HashSet<_>>()
    .len()
}

fn visit_next(
    maze: &Vec<Vec<char>>,
    pos: &(usize, usize),
    direction: &'static Direction,
) -> Option<((usize, usize), &'static Direction)> {
    let (x, y) = add(pos, direction.get_dir())?;
    let new_char = maze.get(y)?.get(x)?;

    match new_char {
        '#' => visit_next(maze, pos, &direction.next()),
        _ => Option::from(((x, y), direction)),
    }
}

fn as_maze(lines: &[String]) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|line| line.chars().map(|char| char).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn add((x, y): &(usize, usize), (x_diff, y_diff): &(i32, i32)) -> Option<(usize, usize)> {
    let new_x = usize::try_from(*x as i32 + x_diff);
    let new_y = usize::try_from(*y as i32 + y_diff);

    match (new_x, new_y) {
        (Ok(new_x_val), Ok(new_y_val)) => Some((new_x_val, new_y_val)),
        _ => None,
    }
}
