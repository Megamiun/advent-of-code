use crate::util::Index2D;
use crate::y2024::util::bounded::{Bounded, Direction};
use rustc_hash::{FxHashMap, FxHashSet};
use std::fmt::Debug;

type Key = (Index2D, Direction);

pub fn get_min_path(lines: &[String]) -> usize {
    Bounded::from(lines).get_min_path()
}

pub fn get_unique_best_spots(lines: &[String]) -> usize {
    Bounded::from(lines).get_unique_best_spots()
}

impl Bounded<char> {
    fn get_min_path(&self) -> usize {
        let min_spanning_tree = self.get_min_spanning_tree();
        let end = self.find_first(&'E').unwrap();

        Direction::VALUES.iter()
            .filter_map(|&&dir| Some(min_spanning_tree.get(&(end, dir))?.0))
            .min().unwrap_or(0)
    }

    fn get_unique_best_spots(&self) -> usize {
        let end = self.find_first(&'E').unwrap();
        
        self.get_unique_spots_on(&mut self.get_min_spanning_tree(), &end)
    }

    fn get_min_spanning_tree(&self) -> FxHashMap<Key, (usize, Vec<Key>)> {
        let mut to_visit = PriorityQueue::<(usize, Key, Key)>::new();
        let mut min_distances_path = FxHashMap::<Key, (usize, Vec<Key>)>::default();
        let start = self.find_first(&'S').unwrap();

        to_visit.push(&(0, (start, *Direction::RIGHT), (start, *Direction::RIGHT)));

        while !to_visit.is_empty() {
            let (score, from_key, to_key) = to_visit.pop().unwrap();

            let (to, dir) = to_key;
            if self.find_safe(&to) == '#' {
                continue;
            }

            if let Some((distance, back)) = min_distances_path.get_mut(&to_key) {
                if *distance == score {
                    back.push(from_key);
                }
                continue;
            }

            min_distances_path.insert(to_key, (score, vec![from_key]));
            to_visit.push(&(score + 1, to_key, ((to + dir.dir).unwrap(), dir)));

            Direction::VALUES.iter()
                .filter(|&&new_dir| new_dir != &dir)
                .for_each(|&new_dir| to_visit.push(&(score + 1001, to_key, ((to + new_dir.dir).unwrap(), *new_dir))))
        }
    
        min_distances_path
    }

    fn get_unique_spots_on(&self, min_distances_path: &mut FxHashMap<Key, (usize, Vec<Key>)>, end: &Index2D) -> usize {
        let min_distance = Direction::VALUES.iter()
            .filter_map(|&&dir| Some(min_distances_path.get(&(*end, dir))?.0))
            .min().unwrap_or(0);

        let mut acc = FxHashSet::<Index2D>::with_capacity_and_hasher(self.width * self.height, Default::default());

        Direction::VALUES.iter().filter(|&&&dir| {
            min_distances_path
                .get(&(*end, dir))
                .is_some_and(|v| v.0 == min_distance)
        }).for_each(|&&dir| Self::capture_path(&mut acc, &min_distances_path, &(*end, dir)));

        acc.len()
    }

    fn capture_path(acc: &mut FxHashSet<Index2D>, min_spanning_tree: &FxHashMap<Key, (usize, Vec<Key>)>, to: &Key) {
        acc.insert(to.0);

        if let Some((_, previous)) = min_spanning_tree.get(to) {
            for prev in previous {
                if prev != to {
                    Self::capture_path(acc, min_spanning_tree, prev)
                }
            }
        }
    }
}

struct PriorityQueue<T: PartialOrd + Ord + Clone> {
    delegate: Vec<T>,
}

impl<T: PartialOrd + Ord + Clone + Debug> PriorityQueue<T> {
    fn new() -> PriorityQueue<T> {
        PriorityQueue { delegate: Vec::new() }
    }

    fn push(&mut self, item: &T) {
        self.push_ordered(item, 0, self.delegate.iter().len())
    }

    fn push_ordered(&mut self, item: &T, start: usize, end: usize) {
        if start >= end {
            self.delegate.insert(start, item.clone());
            return;
        }

        let half = (end + start) / 2;

        if item == &self.delegate[half] {
            self.push_ordered(item, half, half);
        } else if item >= &self.delegate[half] {
            self.push_ordered(item, start, half);
        } else {
            self.push_ordered(item, half + 1, end);
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.delegate.pop()
    }

    fn is_empty(&self) -> bool {
        self.delegate.is_empty()
    }
}
