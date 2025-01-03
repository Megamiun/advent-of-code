use crate::util::coordinates::Index2D;
use crate::util::direction::Direction;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct ArrayBounded<T, const WIDTH: usize, const HEIGHT: usize> {
    pub content: [[T; WIDTH]; HEIGHT]
}

impl<T: PartialEq, const WIDTH: usize, const HEIGHT: usize> Index<&Index2D> for ArrayBounded<T, WIDTH, HEIGHT> {
    type Output = T;

    fn index(&self, index: &Index2D) -> &Self::Output {
        self.find_safe(index)
    }
}

impl<T: PartialEq, const WIDTH: usize, const HEIGHT: usize> IndexMut<&Index2D> for ArrayBounded<T, WIDTH, HEIGHT> {
    fn index_mut(&mut self, index: &Index2D) -> &mut Self::Output {
        self.find_safe_mut(index)
    }
}

impl<T: PartialEq, const WIDTH: usize, const HEIGHT: usize> ArrayBounded<T, WIDTH, HEIGHT> {
    pub fn find(&self, coord: &Index2D) -> Option<&T> {
        self.content.get(coord.1)?.get(coord.0)
    }

    pub fn find_safe(&self, coord: &Index2D) -> &T {
        &self.content[coord.1][coord.0]
    }

    pub fn find_safe_mut(&mut self, coord: &Index2D) -> &mut T {
        &mut self.content[coord.1][coord.0]
    }

    pub fn find_adjacent_with_content_iter<'a>(&'a self, index: &'a Index2D) -> impl Iterator<Item=(Index2D, &'a T)> {
        Direction::VALUES.iter()
            .filter_map(|dir| *index + dir)
            .filter_map(|adj| Some((adj, self.find(&adj)?)))
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
