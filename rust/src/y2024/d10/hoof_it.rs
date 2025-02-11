use crate::util::bounded::Bounded;
use crate::util::coordinates::Index2D;
use itertools::Itertools;

#[allow(dead_code)]
pub fn get_reachable_sum_for(lines: &[String]) -> usize {
    Bounded::from(lines).get_reachable_sum_for()
}

#[allow(dead_code)]
pub fn get_trails_sum_for(lines: &[String]) -> usize {
    Bounded::from(lines).get_trails_sum_for()
}

impl Bounded<char> {
    fn get_reachable_sum_for(&self) -> usize {
        self.find_all_iter(&'0')
            .map(|position| self.get_reachable(0, &position).iter().unique().count())
            .sum()
    }
    
    fn get_trails_sum_for(&self) -> usize {
        self.find_all_iter(&'0')
            .flat_map(|position| self.get_reachable(0, &position))
            .count()
    }

    fn get_reachable(&self, curr: u32, position: &Index2D) -> Vec<Index2D> {
        if curr == 9 {
            return vec!(*position);
        }

        let next = curr + 1;
        let next_char = char::from_digit(next, 10).unwrap();

        self.find_adjacent_with_content(position)
            .filter(|(_, content)| *content == &next_char)
            .flat_map(|(adj, _)| self.get_reachable(next, &adj))
            .collect()
    }
}
