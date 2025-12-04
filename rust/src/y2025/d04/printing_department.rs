use crate::util::bounded::Bounded;
use crate::util::coordinates::Index2D;
use itertools::Itertools;
use rustc_hash::FxHashSet;

#[allow(dead_code)]
pub fn get_accessible_rolls(lines: &[String]) -> usize {
    Bounded::from(lines).get_accessible_rolls()
}

#[allow(dead_code)]
pub fn get_all_accessible_rolls(lines: &[String]) -> usize {
    Bounded::from(lines).get_all_accessible_rolls()
}

impl Bounded<char> {
    fn get_accessible_rolls(&self) -> usize {
        self.get_accessible_rolls_with_exclusions(&FxHashSet::default()).iter().count()
    }

    fn get_all_accessible_rolls(&self) -> usize {
        self.get_all_accessible_rolls_with_exclusions(&mut FxHashSet::default())
    }

    fn get_all_accessible_rolls_with_exclusions(&self, excluded: &mut FxHashSet<Index2D>) -> usize {
        let accessible = self.get_accessible_rolls_with_exclusions(excluded);

        if accessible.is_empty() {
            return 0;
        }

        excluded.extend(&accessible);

        accessible.len() + self.get_all_accessible_rolls_with_exclusions(excluded)
    }

    fn get_accessible_rolls_with_exclusions(&self, removed: &FxHashSet<Index2D>) -> Vec<Index2D> {
        self.find_all_iter(&'@').filter(|position| {
            if removed.contains(position) {
                return false;
            }

            let accessible_rolls = self
                .find_adjacent_and_diagonal_with_content(position)
                .filter(|(cell_position, &content)| content == '@' && !removed.contains(cell_position))
                .count();

            accessible_rolls < 4
        }).collect_vec()
    }
}
