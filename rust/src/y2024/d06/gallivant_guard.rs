use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

struct Direction {
    dir: (i32, i32),
    next: fn() -> &'static Direction,
}

impl Direction {
    const N: &'static Direction = &Direction { dir: (0, -1), next: || Direction::E };
    const E: &'static Direction = &Direction { dir: (1, 0), next: || Direction::S };
    const S: &'static Direction = &Direction { dir: (0, 1), next: || Direction::W };
    const W: &'static Direction = &Direction { dir: (-1, 0), next: || Direction::N };

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

fn find_initial_guard_position(maze: &Vec<Vec<char>>) -> ((usize, usize), &'static Direction) {
    (0..maze.len()).flat_map(|y| {
        (0..maze[y].len())
            .filter(|&x| !['.', '#'].contains(&maze[y][x]))
            .map(|x| ((x, y), Direction::from(maze[y][x])))
            .nth(0)
    })
    .nth(0)
    .unwrap()
}

fn visit_next(
    maze: &Vec<Vec<char>>,
    pos: &(usize, usize),
    direction: &'static Direction,
) -> Option<((usize, usize), &'static Direction)> {
    let (x, y) = add(pos, &direction.dir)?;
    let new_char = maze.get(y)?.get(x)?;

    match new_char {
        '#' => visit_next(maze, pos, (direction.next)()),
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
