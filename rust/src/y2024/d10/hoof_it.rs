use crate::util::Index2D;
use crate::y2024::util::bounded::Bounded;
use std::collections::HashSet;

pub fn get_reachable_sum_for(lines: &[String]) -> usize {
    Bounded::from(lines).get_reachable_sum_for()
}

pub fn get_trails_sum_for(lines: &[String]) -> usize {
    Bounded::from(lines).get_trails_sum_for()
}

impl Bounded<char> {
    fn get_reachable_sum_for(&self) -> usize {
        self.find_all(&'0').iter()
            .map(|position| self.get_reachable(0, position).iter().collect::<HashSet<_>>().len())
            .sum()
    }
    fn get_trails_sum_for(&self) -> usize {
        self.find_all(&'0').iter()
            .flat_map(|position| self.get_reachable(0, position))
            .count()
    }

    fn get_reachable(&self, curr: u32, position: &Index2D) -> Vec<Index2D> {
        if curr == 9 {
            return vec!(*position);
        }

        let next = curr + 1;
        let next_char = char::from_digit(next, 10).unwrap();

        self.find_adjacent(position)
            .iter().filter(|&adj| self.find(*adj).is_some_and(|x| *x == next_char))
            .flat_map(|adj| self.get_reachable(next, adj))
            .collect()
    }
}