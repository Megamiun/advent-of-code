use crate::util::{Diff, Index2D};
use crate::y2024::util::bounded::Bounded;
use crate::y2024::util::collections::key_indexed::key_priority_queue::KeyPriorityQueue;
use crate::y2024::util::direction::Direction;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::collections::{HashMap, HashSet};

pub fn find_all_shortcuts_that_save_at_least(lines: &[String], limit: usize, over: usize) -> usize {
    Bounded::from(lines)
        .into_distance_map()
        .get_all_cheats(limit)
        .iter()
        .filter(|(distance, _)| *distance >= over)
        .count()
}

impl Bounded<char> {
    fn into_distance_map(self) -> Bounded<Option<usize>> {
        Bounded::from_map(self.width, self.height, &self.populate_distances())
    }

    fn populate_distances(&self) -> HashMap<Index2D, usize, FxBuildHasher> {
        let mut distances = self.create_map();
        let start = self.find_first(&'S').unwrap();

        let mut to_visit = KeyPriorityQueue::<(usize, Index2D)>::new();
        to_visit.push(&(0, start));

        while !to_visit.is_empty() {
            let (score, curr) = to_visit.pop().unwrap();

            if self.find_safe(&curr) == '#' || distances.contains_key(&curr) {
                continue;
            }

            distances.insert(curr, score);

            for adj in self.find_adjacent(&curr) {
                to_visit.push(&(score + 1, adj));
            }
        }

        distances
    }

    fn create_map(&self) -> HashMap<Index2D, usize, FxBuildHasher> {
        FxHashMap::with_capacity_and_hasher(self.width * self.height, Default::default())
    }
}

impl Bounded<Option<usize>> {
    fn get_all_cheats(&self, limit: usize) -> Vec<(usize, (Index2D, Index2D))> {
        let diffs = Direction::VALUES.iter()
            .flat_map(|dir| Self::generate_outwards(&Diff(0, 0), 0, limit, *dir))
            .filter(|(_, dist)| *dist != 0)
            .collect::<HashSet<_>>();
        
        self.get_all_coordinates_with_content().iter()
            .filter_map(|(index, &s)| Some((*index, s?)))
            
            .flat_map(|entry| diffs.iter().map(|diff| (entry, *diff)).collect::<Vec<_>>())
            .filter_map(|((destination, dest_dist), (diff, distance))| {
                let source = (destination + diff)?;
                let shortcut_length = dest_dist.checked_sub((*self.find(&source)?)? + distance)?;
                Some((shortcut_length, (source, destination)))
            })
            .filter(|(length, _)| *length > 0)
            .collect()
    }

    fn generate_outwards(diff: &Diff, distance: usize, remaining: usize, to: Direction) -> Vec<(Diff, usize)> {
        if remaining == 0 {
            return vec![(*diff, distance)];
        }

        [to, to.get_clockwise()].iter().flat_map(|dir| 
            Self::generate_outwards(&(diff + dir.get_dir()), distance + 1, remaining - 1, to)
        ).chain([(*diff, distance)]).collect()
    }
}
