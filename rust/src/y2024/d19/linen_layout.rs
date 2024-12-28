use itertools::Itertools;
use rustc_hash::FxHashMap;

#[allow(dead_code)]
pub fn get_possible_towels([available, goals]: &[&[String]; 2]) -> usize {
    let available = &available[0].split(", ").collect_vec();

    Solver::new(available)
        .iterate_valid_arrangement_count(goals)
        .filter(|&arrangements| arrangements > 0)
        .count()
}

#[allow(dead_code)]
pub fn get_towels_arrangements([available, goals]: &[&[String]; 2]) -> usize {
    let available = &available[0].split(", ").collect_vec();

    Solver::new(available)
        .iterate_valid_arrangement_count(goals)
        .sum()
}

struct Solver {
    cache: FxHashMap<String, usize>,
    towels: Vec<String>,
}

impl Solver {
    fn new(available: &[&str]) -> Solver {
        Solver {
            cache: FxHashMap::default(),
            towels: available.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn iterate_valid_arrangement_count<'a>(&'a mut self, goals: &'a [String]) -> impl Iterator<Item=usize> + 'a {
        goals.iter().map(|goal| self.count_valid_arrangements(goal))
    }

    fn count_valid_arrangements(&mut self, goal: &str) -> usize {
        if goal.is_empty() {
            return 1
        }   
        
        if let Some(size) = self.cache.get(goal) {
            return *size
        }
        
        let valid = self.towels.iter()
            .filter_map(|towel| goal.strip_prefix(towel))
            .collect_vec();
        
        let possible_arrangements = valid.iter()
            .map(|remaining| self.count_valid_arrangements(remaining))
            .sum();

        self.cache.insert(goal.to_string(), possible_arrangements);
        possible_arrangements
    }
}
