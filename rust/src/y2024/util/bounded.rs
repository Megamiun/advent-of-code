use crate::util::{Diff, Index2D};
use std::sync::LazyLock;

pub struct Bounded<T> {
    pub content: Vec<Vec<T>>,
    pub height: usize,
    pub width: usize,
}

impl From<&[String]> for Bounded<char> {
    fn from(content: &[String]) -> Bounded<char> {
        let new_content = content.iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();

        Bounded {
            content: new_content,
            height: content.len(),
            width: content[0].len(),
        }
    }
}

impl<T: PartialEq> Bounded<T> {
    pub fn find(&self, coord: &Index2D) -> Option<&T> {
        self.content.get(coord.1)?.get(coord.0)
    }

    pub fn is_within(&self, Index2D(x, y): &Index2D) -> bool {
        *x < self.width && *y < self.height
    }

    pub fn find_first(&self, item: &T) -> Option<Index2D> {
        self.find_all(item).get(0).copied()
    }

    pub fn find_all(&self, item: &T) -> Vec<Index2D> {
        (0..self.height).flat_map(|y| {
            (0..self.width)
                .filter(move |&x| self.content[y][x] == *item)
                .map(move |x| Index2D(x, y))
        }).collect()
    }

    pub fn find_adjacent(&self, index: &Index2D) -> Vec<Index2D> {
        Direction::VALUES.iter()
            .filter_map(|dir| index + dir.dir)
            .collect()
    }
    
    pub fn get_all_coordinates(&self) -> Vec<Index2D> {
        self.content.iter().enumerate()
            .flat_map(|(y, line)|
                line.iter().enumerate()
                    .map(move |(x, _)| Index2D(x, y))
            ).collect::<Vec<_>>()
    }
}

impl<T: Clone> Bounded<T> {
    pub fn find_safe(&self, coord: &Index2D) -> T {
        self.content[coord.1][coord.0].clone()
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Direction {
    pub dir: Diff
}

impl Direction {
    pub const UP: &'static Direction = &Direction { dir: Diff(0, -1) };
    pub const RIGHT: &'static Direction = &Direction { dir: Diff(1, 0) };
    pub const DOWN: &'static Direction = &Direction { dir: Diff(0, 1) };
    pub const LEFT: &'static Direction = &Direction { dir: Diff(-1, 0) };

    pub const VALUES: LazyLock<[&'static Direction; 4]> = LazyLock::new(|| {
        [Direction::UP, Direction::RIGHT, Direction::DOWN, Direction::LEFT]
    });
}
