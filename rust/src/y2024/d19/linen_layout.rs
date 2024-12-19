use rustc_hash::FxHashMap;

pub fn get_possible_towels(lines: &[&[String]]) -> usize {
    let available = &lines[0][0].split(", ").collect::<Vec<_>>();
    let goals = lines[1];

    Solver::new(available)
        .count_valid_arrangements_for_all(goals)
        .iter().filter(|&&arrangements| arrangements > 0)
        .count()
}

pub fn get_towels_arrangements(lines: &[&[String]]) -> usize {
    let available = &lines[0][0].split(", ").collect::<Vec<_>>();
    let goals = lines[1];

    Solver::new(available)
        .count_valid_arrangements_for_all(goals)
        .iter().sum()
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

    fn count_valid_arrangements_for_all(&mut self, goals: &[String]) -> Vec<usize> {
        goals.iter().map(|goal| self.count_valid_arrangements(goal)).collect()
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
            .collect::<Vec<_>>();
        
        let possible_arrangements = valid.iter()
            .map(|remaining| self.count_valid_arrangements(remaining))
            .sum();

        self.cache.insert(goal.to_string(), possible_arrangements);
        possible_arrangements
    }
}
