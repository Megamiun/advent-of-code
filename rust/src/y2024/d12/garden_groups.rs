use crate::util::Index2D;
use crate::y2024::util::bounded::Bounded;
use crate::y2024::util::direction::Direction;
use rustc_hash::FxHashSet;
use std::collections::{HashMap, LinkedList};
use std::iter::successors;
use std::ops::Add;
use std::sync::LazyLock;

pub fn find_price_by_perimeter(lines: &[String]) -> usize {
    Bounded::from(lines).find_price_by_perimeter()
}

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

        for coord in &self.get_all_coordinates() {
            if regions.iter().any(|region| region.contained.contains(coord)) {
                continue
            }
            regions.push(self.capture_regions(coord))
        };

        regions
    }

    fn capture_regions(&self, coord: &Index2D) -> Region {
        let mut contained = FxHashSet::<Index2D>::default();
        let mut barriers = FxHashSet::<Barrier>::default();

        let mut to_visit = LinkedList::<(Index2D, Direction)>::new();

        contained.insert(*coord);
        Direction::VALUES.iter()
            .for_each(|dir| to_visit.push_back((*coord, *dir)));

        let first = self.find_safe(coord);

        while !to_visit.is_empty() {
            let (curr, dir) = to_visit.pop_front().unwrap();
            let next = &curr + dir.get_dir();

            if next.is_none() {
                barriers.insert(Barrier::from(curr, dir));
                continue
            }

            let next = next.unwrap();
            let next_char = self.find(&next);
            if next_char.is_none() || *next_char.unwrap() != first {
                barriers.insert(Barrier::from(curr, dir));
                continue
            }

            if contained.contains(&next) {
                continue
            }

            contained.insert(next);
            Direction::VALUES.iter()
                .for_each(|dir| to_visit.push_back((next, *dir)))
        }

        Region { contained, barriers }
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
        let barriers = Direction::PARALLEL[&barrier.to].iter()
            .flat_map(|dir| {
                successors(Some(barrier.clone()), |curr| curr + dir)
                    .take_while(|next| self.barriers.contains(next))
            }).collect::<FxHashSet<Barrier>>();

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

impl Direction {
    const PARALLEL: LazyLock<HashMap<Direction, Vec<Direction>>> =
        LazyLock::new(|| {
            HashMap::from([
                (Direction::Up, vec![Direction::Left, Direction::Right]),
                (Direction::Down, vec![Direction::Left, Direction::Right]),
                (Direction::Left, vec![Direction::Up, Direction::Down]),
                (Direction::Right, vec![Direction::Up, Direction::Down]),
            ])
        });
}
