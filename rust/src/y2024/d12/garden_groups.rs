use crate::util::bounded::Bounded;
use crate::util::coordinates::Index2D;
use crate::util::direction::Direction;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;
use std::iter::successors;
use std::ops::Add;

#[allow(dead_code)]
pub fn find_price_by_perimeter(lines: &[String]) -> usize {
    Bounded::from(lines).find_price_by_perimeter()
}

#[allow(dead_code)]
pub fn find_price_by_sides(lines: &[String]) -> usize {
    Bounded::from(lines).find_price_by_sides()
}

impl Bounded<char> {
    fn find_price_by_perimeter(&self) -> usize {
        self.find_regions().iter()
            .map(|region| region.barriers.len() * region.contained.len())
            .sum()
    }

    fn find_price_by_sides(&self) -> usize {
        self.find_regions().iter()
            .map(|region| region.find_sides().len() * region.contained.len())
            .sum()
    }

    fn find_regions(&self) -> Vec<Region> {
        let mut regions = Vec::<Region>::new();

        for (coord, content) in self.get_all_coordinates_with_content_iter() {
            if regions.iter().any(|region| region.group == *content && region.contained.contains(&coord)) {
                continue
            }
            regions.push(self.capture_regions(&coord))
        };

        regions
    }

    fn capture_regions(&self, coord: &Index2D) -> Region {
        let mut contained = FxHashSet::<Index2D>::with_capacity_and_hasher(60, Default::default());
        let mut barriers = FxHashSet::<Barrier>::with_capacity_and_hasher(45, Default::default());

        let mut to_visit = VecDeque::<(Index2D, Direction)>::new();

        contained.insert(*coord);
        Direction::VALUES.iter()
            .for_each(|dir| to_visit.push_back((*coord, *dir)));

        let group = self.find_safe(coord);

        while !to_visit.is_empty() {
            let (curr, dir) = to_visit.pop_back().unwrap();
            let next = &curr + dir;

            if next.is_none() {
                barriers.insert(Barrier::from(curr, dir));
                continue
            }

            let next = next.unwrap();
            let next_char = self.find(&next);
            if next_char.is_none() || next_char.unwrap() != group {
                barriers.insert(Barrier::from(curr, dir));
                continue
            }

            if !contained.insert(next) {
                continue
            }
            
            Direction::VALUES.iter()
                .for_each(|dir| to_visit.push_back((next, *dir)))
        }

        Region { contained, barriers, group: *group }
    }
}

#[derive(PartialEq, Eq)]
struct Side {
    barriers: FxHashSet<Barrier>
}

#[derive(PartialEq, Eq)]
struct Region {
    contained: FxHashSet<Index2D>,
    barriers: FxHashSet<Barrier>,
    group: char
}

impl Region {
    fn find_sides(&self) -> Vec<Side> {
        let mut sides = Vec::<Side>::new();

        for barrier in &self.barriers {
            if sides.iter().any(|side| side.barriers.contains(&barrier)) {
                continue
            }
            sides.push(self.capture_sides(&barrier))
        };

        sides
    }

    fn capture_sides(&self, barrier: &Barrier) -> Side {
        let barriers = [barrier.to.get_clockwise(), barrier.to.get_counter_clockwise()].iter()
            .flat_map(|dir| {
                successors(Some(*barrier), |curr| curr + dir)
                    .take_while(|next| self.barriers.contains(next))
            }).collect();

        Side { barriers }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Barrier {
    position: Index2D,
    to: Direction,
}

impl Barrier {
    fn from(position: Index2D, to: Direction) -> Barrier {
        Barrier { position, to }
    }
}

impl Add<&Direction> for &Barrier {
    type Output = Option<Barrier>;

    fn add(self, rhs: &Direction) -> Self::Output {
        Some(Barrier { position: (self.position + rhs.get_dir())?, to: self.to })
    }
}
