use crate::util::{Diff, Index2D};
use crate::y2024::util::bounded::EnumDirection::{Down, Left, Right, Up};
use std::fmt::Debug;
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

impl<T: Clone> From<&Vec<Vec<T>>> for Bounded<T> {
    fn from(content: &Vec<Vec<T>>) -> Bounded<T> {
        Bounded {
            content: content.clone(),
            height: content.len(),
            width: content[0].len(),
        }
    }
}

impl<T: PartialEq + Clone> Bounded<T> {
    pub fn create_from(map: &[String], get_cell: fn(char) -> T) -> Bounded<T> {
        let new_map = map.iter().map(|line|
            line.chars().map(|c| get_cell(c)).collect::<Vec<_>>()
        ).collect::<Vec<_>>();

        Bounded::from(&new_map)
    }

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

    pub fn get_all_coordinates_with_content(&self) -> Vec<(Index2D, &T)> {
        self.content.iter().enumerate()
            .flat_map(|(y, line)|
                line.iter().enumerate()
                    .map(move |(x, item)| (Index2D(x, y), item))
            ).collect::<Vec<_>>()
    }

    pub fn print_by(&self, get_content: &dyn for<'b> Fn(&'b Index2D, &'b T) -> String) {
        self.content.iter().enumerate().for_each(|(y, line)| {
            line.iter().enumerate()
                .for_each(move |(x, item)| print!("{}", get_content(&Index2D(x, y), item)));
            println!()
        })
    }

    pub fn set(&mut self, Index2D(x, y): &Index2D, value: T) {
        self.content[*y][*x] = value
    }
}

impl<T: Clone> Bounded<T> {
    pub fn find_safe(&self, coord: &Index2D) -> T {
        self.content[coord.1][coord.0].clone()
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Ord, PartialOrd)]
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


#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Ord, PartialOrd)]
pub enum EnumDirection {
    Up,
    Right,
    Down,
    Left
}

impl EnumDirection {
    pub const VALUES: [EnumDirection; 4] = [Up, Right, Down, Left];
    
    pub fn get_dir(&self) -> Diff {
        match self {
            Up => Diff(0, -1),
            Right =>  Diff(1, 0),
            Down => Diff(0, 1),
            Left =>  Diff(-1, 0)
        }
    }

    pub fn get_reverse(&self) -> EnumDirection {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right
        }
    }

    pub fn get_clockwise(&self) -> EnumDirection {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }
    
    pub fn get_counter_clockwise(&self) -> EnumDirection {
        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down
        }
    }
}
