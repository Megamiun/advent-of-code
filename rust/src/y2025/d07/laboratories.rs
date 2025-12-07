use crate::util::bounded::Bounded;
use crate::util::coordinates::Index2D;
use crate::util::direction::Direction;
use crate::util::direction::Direction::Down;
use rustc_hash::FxHashMap;
use Direction::{Left, Right};

#[allow(dead_code)]
pub fn get_hit_splitters(lines: &[String]) -> usize {
    Bounded::from(lines).get_hit_splitters().len()
}

#[allow(dead_code)]
pub fn get_possibility_count(lines: &[String]) -> usize {
    Bounded::from(lines).get_possibilities_count()
}

impl Bounded<char> {
    fn get_hit_splitters(&self) -> FxHashMap<Index2D, usize> {
        let mut splitters = FxHashMap::default();
        self.visit(&mut splitters, self.find_first(&'S').unwrap(), Down);

        splitters
    }

    fn get_possibilities_count(&self) -> usize {
        self.visit(&mut FxHashMap::default(), self.find_first(&'S').unwrap(), Down)
    }

    fn visit(&self, splitters: &mut FxHashMap<Index2D, usize>, origin: Index2D, direction: Direction) -> usize {
        let current = (origin + direction).unwrap();
        match self.find(&current) {
            Some('^') => {
                if splitters.contains_key(&current) {
                    return splitters[&origin];
                }

                let left = self.visit(splitters, current, Left);
                let right = self.visit(splitters, current, Right);

                splitters.insert(current, left + right);
                left + right
            }
            None => 1,
            _ => self.visit(splitters, current, Down),
        }
    }
}
