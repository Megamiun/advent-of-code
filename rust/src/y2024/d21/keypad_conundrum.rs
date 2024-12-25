use crate::util::coordinates::Index2D;
use crate::util::direction::Direction;
use crate::util::direction::Direction::{Down, Left, Right, Up};
use rustc_hash::FxHashMap;
use std::cmp::min;
use std::collections::HashMap;
use std::sync::LazyLock;

pub fn get_sum_of_complexity(lines: &[String], robots: usize) -> usize {
    lines.iter().map(|line| Solver::new().get_complexity(line, robots)).sum()
}

struct Solver {
    cache: FxHashMap<(String, usize), usize>,
}

impl Solver {
    fn new() -> Solver {
        Solver { cache: FxHashMap::default() }
    }
    
    fn get_complexity(&mut self, line: &String, robots: usize) -> usize {
        let numeric = NUMERIC.to_owned();
        let directional = DIRECTIONAL.to_owned();
        let layers = [&numeric].iter().chain(vec![&directional; robots].iter())
            .copied().collect::<Vec<_>>();

        let numerical_part = line.strip_suffix('A').unwrap();
        let numerical_value = usize::from_str_radix(numerical_part, 10).unwrap();
        let minimum = self.solve_for(numerical_part, &layers.as_slice(), 0);

        minimum * numerical_value
    }
    
    fn solve_for(&mut self, line: &str, map: &[&HashMap<char, Index2D>], level: usize) -> usize {
        if let Some(cached) = self.cache.get(&(line.to_string(), level)) {
            return *cached
        }
        
        if map.is_empty() {
            return line.len() + 1
        }

        let result = ("A".to_string() + line + "A").chars().collect::<Vec<_>>().windows(2).map(|window| {
            let [horizontal, vertical] = self.derive_movement(window, map[0]);

            if BANNED.contains(&(window[1], window[0])) {
                return self.solve_for(&join_into(&vertical, &horizontal), &map[1..], level + 1)
            } else if BANNED.contains(&(window[0], window[1])) {
                return self.solve_for(&join_into(&horizontal, &vertical), &map[1..], level + 1)
            }

            min(
                self.solve_for(&join_into(&horizontal, &vertical), &map[1..], level + 1),
                self.solve_for(&join_into(&vertical, &horizontal), &map[1..], level + 1),
            )
        }).sum();
        
        self.cache.insert((line.to_string(), level), result);
        
        result
    }

    fn derive_movement(&self, window: &[char], position: &HashMap<char, Index2D>) -> [String; 2] {
        let distance = position[&window[1]] - position[&window[0]];

        let horizontal = if distance.0 > 0 { Right } else { Left };
        let vertical = if distance.1 > 0 { Down } else { Up };

        [
            String::from_iter((0..distance.0.abs()).map(|_| horizontal.into_char())),
            String::from_iter((0..distance.1.abs()).map(|_| vertical.into_char()))
        ]
    }
}

fn join_into(horizontal: &str, vertical: &str) -> String {
    [vertical, horizontal].join("")
}

const NUMERIC: LazyLock<HashMap<char, Index2D>> = LazyLock::new(|| HashMap::from([
    ('A', Index2D(2, 3)),
    ('0', Index2D(1, 3)),
    ('1', Index2D(0, 2)),
    ('2', Index2D(1, 2)),
    ('3', Index2D(2, 2)),
    ('4', Index2D(0, 1)),
    ('5', Index2D(1, 1)),
    ('6', Index2D(2, 1)),
    ('7', Index2D(0, 0)),
    ('8', Index2D(1, 0)),
    ('9', Index2D(2, 0)),
]));

const DIRECTIONAL: LazyLock<HashMap<char, Index2D>> = LazyLock::new(|| HashMap::from([
    ('<', Index2D(0, 1)),
    ('v', Index2D(1, 1)),
    ('>', Index2D(2, 1)),
    ('^', Index2D(1, 0)),
    ('A', Index2D(2, 0)),
]));

const BANNED: &[(char, char)] = &[
    ('A', '1'),
    ('A', '4'),
    ('A', '7'),
    ('0', '1'),
    ('0', '4'),
    ('0', '7'),
    ('A', '<'),
    ('^', '<'),
];

impl Direction {
    fn into_char(self) -> char {
        self.into()
    }
}
