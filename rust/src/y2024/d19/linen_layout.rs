use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::cell::RefCell;

#[allow(dead_code)]
pub fn get_possible_towels([available, goals]: &[&[String]; 2]) -> usize {
    let available = &available[0].split(", ").collect_vec();

    Solver::new(available, goals.len())
        .iterate_valid_arrangement_count(goals)
        .filter(|&arrangements| arrangements > 0)
        .count()
}

#[allow(dead_code)]
pub fn get_towels_arrangements([available, goals]: &[&[String]; 2]) -> usize {
    let available = &available[0].split(", ").collect_vec();

    Solver::new(available, goals.len())
        .iterate_valid_arrangement_count(goals)
        .sum()
}

struct Solver<'a> {
    cache: RefCell<FxHashMap<String, usize>>,
    towels: &'a[&'a str],
}

impl<'a> Solver<'a> {
    fn new(available: &'a[&'a str], goals: usize) -> Solver<'a> {
        Solver {
            cache: RefCell::new(FxHashMap::with_capacity_and_hasher(goals * 2, Default::default())),
            towels: available
        }
    }

    fn iterate_valid_arrangement_count(&'a mut self, goals: &'a [String]) -> impl Iterator<Item=usize> + 'a {
        goals.iter().map(|goal| self.count_valid_arrangements(goal))
    }

    fn count_valid_arrangements(&self, goal: &str) -> usize {
        if goal.is_empty() {
            return 1
        }
        
        if let Some(size) = self.cache.borrow().get(goal) {
            return *size
        }
        
        let valid = self.towels.iter()
            .filter_map(|towel| goal.strip_prefix(towel))
            .map(|remaining| self.count_valid_arrangements(remaining))
            .sum();

        self.cache.borrow_mut().insert(goal.to_string(), valid);
        valid
    }
}
