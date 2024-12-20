use crate::util::Index2D;
use crate::y2024::util::direction::Direction;
use std::collections::HashMap;
use std::hash::BuildHasher;

pub struct Bounded<T> {
    pub content: Vec<Vec<T>>,
    pub height: usize,
    pub width: usize,
}

impl From<&[String]> for Bounded<char> {
    fn from(content: &[String]) -> Bounded<char> {
        let new_content = content
            .iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();

        Bounded {
            content: new_content,
            height: content.len(),
            width: content[0].len(),
        }
    }
}

impl<T: Copy> From<&Vec<Vec<T>>> for Bounded<T> {
    fn from(content: &Vec<Vec<T>>) -> Bounded<T> {
        Bounded {
            content: content.clone(),
            height: content.len(),
            width: content[0].len(),
        }
    }
}

impl<T: Copy> Bounded<T> {
    pub fn from_map(width: usize, height: usize, content: &HashMap<Index2D, T, impl BuildHasher>) -> Bounded<Option<T>> {
        let mut new_content = vec![vec![None; width]; height];

        for (Index2D(x, y), value) in content {
            new_content[*y][*x] = Some(*value); 
        }

        Bounded { content: new_content, height, width }
    }
}

impl<T: PartialEq + Copy> Bounded<T> {
    pub fn create_from(map: &[String], get_cell: fn(char) -> T) -> Bounded<T> {
        let new_map = map
            .iter()
            .map(|line| line.chars().map(|c| get_cell(c)).collect::<Vec<_>>())
            .collect::<Vec<_>>();

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
        (0..self.height)
            .flat_map(|y| {
                (0..self.width)
                    .filter(move |&x| self.content[y][x] == *item)
                    .map(move |x| Index2D(x, y))
            })
            .collect()
    }

    pub fn find_adjacent(&self, index: &Index2D) -> Vec<Index2D> {
        Direction::VALUES
            .iter()
            .filter_map(|dir| index + dir.get_dir())
            .filter(|adj| self.is_within(adj))
            .collect()
    }

    pub fn get_all_coordinates(&self) -> Vec<Index2D> {
        self.content
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, _)| Index2D(x, y)))
            .collect::<Vec<_>>()
    }

    pub fn get_all_coordinates_with_content(&self) -> Vec<(Index2D, &T)> {
        self.content
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(x, item)| (Index2D(x, y), item))
            })
            .collect::<Vec<_>>()
    }

    pub fn print_by(&self, get_content: &dyn for<'b> Fn(&'b Index2D, &'b T) -> String) {
        self.content.iter().enumerate().for_each(|(y, line)| {
            line.iter()
                .enumerate()
                .for_each(move |(x, item)| print!("{}", get_content(&Index2D(x, y), item)));
            println!()
        })
    }

    pub fn set(&mut self, Index2D(x, y): &Index2D, value: T) {
        self.content[*y][*x] = value
    }
}

impl<T: Copy> Bounded<T> {
    pub fn find_safe(&self, coord: &Index2D) -> T {
        self.content[coord.1][coord.0]
    }
}
