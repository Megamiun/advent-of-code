use crate::util::coordinates::Index2D;
use crate::util::direction::Direction;
use itertools::Itertools;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Bounded<T> {
    pub content: Vec<Vec<T>>,
    pub height: usize,
    pub width: usize,
}

impl<T: PartialEq> Index<&Index2D> for Bounded<T> {
    type Output = T;

    fn index(&self, index: &Index2D) -> &Self::Output {
        self.find_safe(index)
    }
}

impl<T: PartialEq> IndexMut<&Index2D> for Bounded<T> {
    fn index_mut(&mut self, index: &Index2D) -> &mut Self::Output {
        self.find_safe_mut(index)
    }
}

impl From<&[String]> for Bounded<char> {
    fn from(content: &[String]) -> Bounded<char> {
        let new_content = content.iter()
            .map(|line| line.chars().collect_vec())
            .collect();

        Bounded {
            content: new_content,
            height: content.len(),
            width: content[0].len(),
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Bounded<T> {
    fn from(content: Vec<Vec<T>>) -> Bounded<T> {
        Bounded {
            height: content.len(),
            width: content[0].len(),
            content
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

impl<T: PartialEq + Copy> Bounded<Option<T>> {
    pub fn get_all_present_iter(&self) -> impl Iterator<Item=(Index2D, &T)> + '_ {
        self.content.iter().enumerate().flat_map(|(y, line)| {
            line.iter().enumerate()
                .filter_map(move |(x, item)| Some((Index2D(x, y), item.as_ref()?)))
        })
    }

    pub fn get_all_present(&self) -> Vec<(Index2D, &T)> {
        self.get_all_present_iter().collect()
    }
}

impl<T: PartialEq> Bounded<T> {
    pub fn create_from(map: &[String], get_cell: fn(char) -> T) -> Bounded<T> {
        let new_map = map.iter()
            .map(|line| line.chars().map(|c| get_cell(c)).collect_vec())
            .collect_vec();

        Bounded::from(new_map)
    }

    pub fn find(&self, coord: &Index2D) -> Option<&T> {
        self.content.get(coord.1)?.get(coord.0)
    }

    pub fn find_safe(&self, coord: &Index2D) -> &T {
        &self.content[coord.1][coord.0]
    }

    pub fn find_safe_mut(&mut self, coord: &Index2D) -> &mut T {
        &mut self.content[coord.1][coord.0]
    }

    pub fn is_within(&self, Index2D(x, y): &Index2D) -> bool {
        *x < self.width && *y < self.height
    }

    pub fn find_first(&self, item: &T) -> Option<Index2D> {
        self.find_all_iter(item).nth(0)
    }

    pub fn find_all_iter<'a>(&'a self, item: &'a T) -> impl Iterator<Item=Index2D> + 'a {
        self.content.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate()
                .filter(|(_, curr)| **curr == *item)
                .map(move |(x, _)| Index2D(x, y))
        })
    }

    pub fn find_adjacent(&self, index: &Index2D) -> Vec<Index2D> {
        Direction::VALUES
            .iter()
            .filter_map(|dir| index + dir.get_dir())
            .filter(|adj| self.is_within(adj))
            .collect()
    }

    pub fn find_adjacent_with_dir(&self, index: &Index2D) -> Vec<(Index2D, Direction)> {
        Direction::VALUES
            .iter()
            .filter_map(|dir| Some(((index + dir.get_dir())?, *dir)))
            .filter(|(adj, _)| self.is_within(adj))
            .collect()
    }

    pub fn find_adjacent_with_content<'a>(&'a self, index: &'a Index2D) -> impl Iterator<Item=(Index2D, &'a T)> {
        Direction::VALUES.iter()
            .filter_map(|dir| *index + dir)
            .filter_map(|adj| Some((adj, self.find(&adj)?)))
    }

    pub fn get_all_coordinates_iter(&self) -> impl Iterator<Item=Index2D> + '_ {
        (0..self.height).flat_map(|y|
            (0..self.width).map(move |x| Index2D(x, y)))
    }

    pub fn get_all_coordinates(&self) -> Vec<Index2D> {
        self.get_all_coordinates_iter().collect()
    }

    pub fn get_all_coordinates_with_content_iter(&self) -> impl Iterator<Item=(Index2D, &T)> {
        self.content.iter().enumerate().flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, item)| (Index2D(x, y), item))
        })
    }

    pub fn get_all_coordinates_with_content(&self) -> Vec<(Index2D, &T)> {
        self.get_all_coordinates_with_content_iter().collect()
    }

    pub fn print_by(&self, get_content: &dyn for<'b> Fn(&'b Index2D, &'b T) -> &'b str) {
        self.content.iter().enumerate().for_each(|(y, line)| {
            line.iter()
                .enumerate()
                .for_each(move |(x, item)| print!("{}", get_content(&Index2D(x, y), item)));
            println!()
        })
    }
}
