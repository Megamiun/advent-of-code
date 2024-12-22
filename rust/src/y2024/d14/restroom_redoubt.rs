use crate::util::coordinates::{Diff, Index2D};
use crate::util::bounded::Bounded;
use num_traits::ToPrimitive;
use regex::Regex;
use rustc_hash::FxHashSet;
use std::cell::RefCell;
use std::collections::LinkedList;
use std::iter::successors;
use std::string::ToString;
use std::sync::LazyLock;

type MovingRobot = (Index2D, Diff);

const EXTRACTOR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("(\\d+).(\\d+).{3}([-0-9]+).([-0-9]+)").unwrap());

pub fn get_safety_score(lines: &[String]) -> usize {
    let bounds = Index2D(101, 103);
    let half_x = bounds.0 / 2;
    let half_y = bounds.1 / 2;
    let seconds = 100;

    let map = Bounded::from(bounds);

    let after_move = &lines.iter()
        .map(parse)
        .map(|robot| map.move_robot(&robot, seconds).0)
        .collect::<Vec<_>>();

    count(after_move, &|robot| robot.0 < half_x && robot.1 < half_y) *
        count(after_move, &|robot| robot.0 < half_x && robot.1 > half_y) *
        count(after_move, &|robot| robot.0 > half_x && robot.1 < half_y) *
        count(after_move, &|robot| robot.0 > half_x && robot.1 > half_y)
}

pub fn get_similar_to_tree(lines: &[String]) -> usize {
    let bounds = Index2D(101, 103);

    let initial_position = lines.iter()
        .map(parse)
        .collect::<Vec<_>>();

    let map = RefCell::new(Bounded::from(bounds));

    successors(Some(initial_position), |robots| {
        Some(robots.iter()
            .map(|robot| map.borrow().move_robot(robot, 1))
            .collect::<Vec<_>>())
    }).enumerate()
        .inspect(|(_, robots)| map.borrow_mut().fill(robots))
        .filter(|(_, robots)| map.borrow().has_big_cluster(robots))
        .inspect(|(second, _)| map.borrow().print(*second))
        .nth(0).unwrap().0
}

fn count<T>(positions: &Vec<T>, matches: &dyn Fn(&&T) -> bool) -> usize {
    positions.iter().filter(matches).count()
}

fn parse(line: &String) -> (Index2D, Diff) {
    let (_, [x, y, diff_x, diff_y]) =
        EXTRACTOR.captures(line).unwrap().extract();

    (Index2D(to_usize(x), to_usize(y)), Diff(to_i32(diff_x), to_i32(diff_y)),)
}

fn to_i32(x: &str) -> i32 {
    i32::from_str_radix(x, 10).unwrap()
}

fn to_usize(x: &str) -> usize {
    usize::from_str_radix(x, 10).unwrap()
}

impl Bounded<bool> {
    fn from(bounds: Index2D) -> Bounded<bool> {
        Bounded {
            content: vec![vec![false; bounds.0]; bounds.1],
            height: bounds.1,
            width: bounds.0
        }
    }

    fn move_robot(&self, (previous, diff): &MovingRobot, amount: usize) -> MovingRobot {
        (self.contrains_to(&((diff * amount) + previous)), *diff)
    }

    fn fill(&mut self, robots: &[MovingRobot]) {
        self.get_all_coordinates().iter()
            .for_each(|Index2D(x, y)| self.content[*y][*x] = false);

        robots.iter()
            .for_each(|(Index2D(x, y), _)| self.content[*y][*x] = true);
    }

    fn print(&self, second: usize) {
        println!("=======================================================================================");
        println!("====================================    {second:4}       ====================================");
        println!("=======================================================================================");

        self.print_by(&|_, filled| match *filled {
            true => "◻️".to_string(),
            false => "◼️".to_string()
        })
    }

    fn contrains_to(&self, Diff(diff_x, diff_y): &Diff) -> Index2D {
        let x = diff_x.rem_euclid(self.width.to_i32().unwrap()).to_usize().unwrap();
        let y = diff_y.rem_euclid(self.height.to_i32().unwrap()).to_usize().unwrap();

        Index2D(x, y)
    }

    fn has_big_cluster(&self, robots: &[MovingRobot]) -> bool {
        let cluster_min_size = robots.len() / 3;
        let half = robots.len() / 2;
        
        robots.iter()
            // As we are searching for a big cluster, doing that lightens the load a little 
            // with low risk, as it is highly improbable that the whole cluster is in the first half
            .skip(half)
            .any(|(position, _)| self.capture_group_size(position) > cluster_min_size)
    }

    fn capture_group_size(&self, position: &Index2D) -> usize {
        // This allows us not to do further allocation for lone robots, a pretty common scenario
        let adjacent = self.find_adjacent(position);
        if !adjacent.iter().any(|pos| self.find(pos).is_some_and(|exists| *exists)) {
            return 1;
        }
        
        let mut contained = FxHashSet::<Index2D>::default();
        let mut to_visit = LinkedList::<Index2D>::new();

        contained.insert(*position);
        adjacent.iter().for_each(|pos| to_visit.push_back(*pos));
        
        while !to_visit.is_empty() {
            let curr = to_visit.pop_front().unwrap();
            let exists = self.find(&curr);

            if contained.contains(&curr) || exists.is_none() || !exists.unwrap() {
                continue
            }

            contained.insert(curr);
            self.find_adjacent(&curr).iter()
                .for_each(|next| to_visit.push_back(*next))
        }

        contained.len()
    }
}