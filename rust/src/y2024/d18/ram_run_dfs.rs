use crate::util::Index2D;
use crate::y2024::util::bounded::Bounded;
use crate::y2024::util::collections::key_indexed::key_priority_queue::KeyPriorityQueue;
use rustc_hash::{FxHashMap, FxHashSet};
use std::iter::successors;
use crate::y2024::d18::parse::parse;
use crate::y2024::util::direction::Direction;
use crate::y2024::util::direction::Direction::{Down, Left, Right, Up};

pub fn find_min_blocker_dfs(lines: &[String], dimensions: usize) -> String {
    let to_fall = lines.iter().map(parse).collect::<Vec<_>>();

    let mut prev_path = vec![to_fall[0]];

    for curr in 0..to_fall.len() {
        let falling_stone = to_fall[curr];
        if !prev_path.contains(&falling_stone) {
            continue;
        }

        let map = Bounded::create_using(dimensions, &to_fall[..curr + 1]);
        if let Some(path) = map.get_a_path() {
            prev_path = path
        } else { 
            return format!("{},{}", falling_stone.0, falling_stone.1)
        }
    }
    
    panic!("Path is not blocked")
}

impl Bounded<bool> {
    fn get_a_path(&self) -> Option<Vec<Index2D>> {
        self.get_a_path_rec(&Index2D(0, 0), &Index2D(self.width - 1, self.height - 1), &mut FxHashSet::default())
    }

    fn get_a_path_rec(&self, curr: &Index2D, goal: &Index2D, visited: &mut FxHashSet<Index2D>) -> Option<Vec<Index2D>> {
        let curr_content = self.find(curr);
        if curr_content.is_none() || *curr_content.unwrap() || visited.contains(curr) {
            return None
        }
        
        if curr == goal {
            return Some(vec![*curr])
        }
        
        visited.insert(*curr);
        
        let mut traverse = |a: &Index2D, b: &Direction| {
            Some(self.get_a_path_rec(&(a + b.get_dir())?, goal, visited)?)
        };
        
        let mut vec = traverse(curr, &Right)
            .or_else(|| traverse(curr, &Down))
            .or_else(|| traverse(curr, &Left))
            .or_else(|| traverse(curr, &Up))?;
        
        vec.push(*curr);
        Some(vec)
    }
}