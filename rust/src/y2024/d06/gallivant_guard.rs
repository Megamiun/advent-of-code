use crate::util::bounded::Bounded;
use crate::util::coordinates::Index2D;
use crate::util::direction::Direction;
use rustc_hash::FxHashSet;
use std::collections::HashSet;
use std::hash::BuildHasher;
use std::iter::successors;

impl Direction {
    fn next(&self) -> Direction {
        self.get_clockwise()
    }
}

#[allow(dead_code)]
pub fn get_visited_count(lines: &[String]) -> usize {
    let maze = Bounded::from(lines);
    let (guard_pos, guard_dir) = maze.find_initial_guard_position();

    maze.get_unique_guard_positions(guard_pos, guard_dir).len()
}

#[allow(dead_code)]
pub fn get_loops_after_obstacle(lines: &[String]) -> usize {
    let maze = Bounded::from(lines);
    let (guard_pos, guard_dir) = maze.find_initial_guard_position();

    maze.get_unique_guard_positions(guard_pos, guard_dir).iter()
        .filter(|&obstacle| maze.causes_loop(*obstacle, guard_pos, guard_dir))
        .count()
}

impl Bounded<char> {
    fn get_unique_guard_positions(&self, guard_pos: Index2D, guard_dir: Direction) -> HashSet<Index2D> {
        successors(
            Option::from((guard_pos, guard_dir)),
            |(new_pos, new_dir)| self.visit_next(new_pos, *new_dir),
        ).map(|(position, _)| position)
            .collect()
    }

    fn causes_loop(&self, obstacle: Index2D, guard_pos: Index2D, guard_dir: Direction) -> bool {
        let mut visited = self.create_hash_set_for();
        let mut curr = (guard_pos, guard_dir);

        loop {
            let (position, direction) = curr;
            let maybe_next = self.visit_next_with_obstacle(&position, direction, &obstacle);

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

    fn find_initial_guard_position(&self) -> (Index2D, Direction) {
        self.get_all_coordinates_with_content_iter()
            .filter(|(_, value)| !['.', '#'].contains(value))
            .map(|(index, value)| (index, Direction::from(*value)))
            .nth(0).unwrap()
    }

    fn visit_next_with_obstacle(
        &self,
        pos: &Index2D,
        direction: Direction,
        obstacle: &Index2D,
    ) -> Option<(Index2D, Direction)> {
        let next = (pos + direction.get_dir())?;

        if *obstacle == next {
            return Option::from((*pos, direction.next()));
        }

        self.get_next(pos, direction, &next)
    }

    fn visit_next(&self, pos: &Index2D, direction: Direction) -> Option<(Index2D, Direction)> {
        let next = (pos + direction.get_dir())?;
        self.get_next(pos, direction, &next)
    }

    fn get_next(&self, pos: &Index2D, direction: Direction, next: &Index2D) -> Option<(Index2D, Direction)> {
        match self.find(&next)? {
            '#' => Option::from((*pos, direction.next())),
            _ => Option::from((*next, direction)),
        }
    }

    fn create_hash_set_for(&self) -> HashSet<(Index2D, Direction), impl BuildHasher> {
        let capacity = self.width.pow(2);
        // Default impl makes this slower (From 41ms to 60ms on my computer) for the complete flow
        // HashSet::<(Index2D, &Direction)>::with_capacity(capacity)
        FxHashSet::<(Index2D, Direction)>::with_capacity_and_hasher(capacity, Default::default())
    }
}
