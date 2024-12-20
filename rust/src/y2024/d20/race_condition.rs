use crate::util::{Diff, Index2D};
use crate::y2024::util::bounded::Bounded;
use crate::y2024::util::collections::key_indexed::key_priority_queue::KeyPriorityQueue;
use crate::y2024::util::direction::Direction;
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};
use std::collections::{HashMap, HashSet};
use std::hash::BuildHasher;

pub fn find_all_shortcuts_that_save_at_least(lines: &[String], limit: usize, min_saved: usize) -> usize {
    Bounded::from(lines)
        .into_distance_map()
        .get_cheats_count(limit, min_saved)
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
    fn get_cheats_count(&self, limit: usize, min_saved: usize) -> usize {
        let mut diffs = FxHashSet::<(Diff, usize)>::default();

        Direction::VALUES.iter().for_each(|dir|
            Self::generate_outwards(&mut diffs, &(Diff(0, 0) + dir.get_dir()), 1, limit - 1, *dir));

        self.get_all_coordinates_with_content().iter()
            .filter(|(_, distance)| distance.is_some())
            .flat_map(|index_dist| diffs.iter().map(|diff| (*index_dist, *diff)))
            .filter_map(|((dest, &dest_dist), (diff, distance))| {
                let source = (dest + diff)?;
                let shortcut_value = dest_dist?.checked_sub((*self.find(&source)?)? + distance)?;

                Some((shortcut_value, (source, dest)))
            })
            .filter(|(length, _)| *length >= min_saved)
            .count()
    }

    fn generate_outwards(diffs: &mut HashSet<(Diff, usize), impl BuildHasher>, diff: &Diff, distance: usize, remaining: usize, to: Direction) {
        if !diffs.insert((*diff, distance)) || remaining == 0 {
            return;
        }

        for dir in [to, to.get_clockwise()].iter() {
            Self::generate_outwards(diffs, &(diff + dir.get_dir()), distance + 1, remaining - 1, to)
        }
    }
}
