use crate::util::coordinates::Index2D;
use crate::y2024::d18::parse::parse;
use crate::util::bounded::Bounded;
use crate::util::collections::key_indexed::key_priority_queue::KeyPriorityQueue;
use rustc_hash::FxHashMap;
use std::iter::successors;

#[allow(dead_code)]
pub fn find_min_steps_after(lines: &[String], dimensions: usize, bytes: usize) -> usize {
    let to_fall = lines.iter().map(parse).take(bytes).collect::<Vec<_>>();

    Bounded::create_using(dimensions, &to_fall)
        .get_min_path_for_exit()
        .unwrap().len() - 1
}

#[allow(dead_code)]
pub fn find_min_blocker(lines: &[String], dimensions: usize) -> String {
    let to_fall = lines.iter().map(parse).collect::<Vec<_>>();

    let mut prev_path = vec![to_fall[0]];

    for curr in 0..to_fall.len() {
        let falling_stone = to_fall[curr];
        if !prev_path.contains(&falling_stone) {
            continue;
        }

        let map = Bounded::create_using(dimensions, &to_fall[..curr + 1]);
        if let Some(path) = map.get_min_path_for_exit() {
            prev_path = path
        } else { 
            return format!("{},{}", falling_stone.0, falling_stone.1)
        }
    }
    
    panic!("Path is not blocked")
}

impl Bounded<bool> {
    fn get_min_path_for_exit(&self) -> Option<Vec<Index2D>> {
        let mut visited: FxHashMap<Index2D, Option<Index2D>> = FxHashMap::with_capacity_and_hasher(self.width * self.height, Default::default());
        let mut to_visit = KeyPriorityQueue::<(usize, (Index2D, Option<Index2D>))>::new();
        to_visit.push(&(0, (Index2D(0, 0), None)));

        let end = Index2D(self.width - 1, self.height - 1);
        
        while !to_visit.is_empty() {
            let (score, (curr, previous)) = to_visit.pop().unwrap();

            if *self.find_safe(&curr) || visited.contains_key(&curr) {
                continue;
            }

            visited.insert(curr, previous);

            if curr == end {
                return Some(successors(Some(end), |prev| visited[prev]).collect());
            }

            for adj in self.find_adjacent(&curr) {
                to_visit.push(&(score + 1, (adj, Some(curr))));
            }
        }

        None
    }
}
