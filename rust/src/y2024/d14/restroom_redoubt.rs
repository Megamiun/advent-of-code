use crate::util::bounded::Bounded;
use crate::util::coordinates::{Diff, Index2D};
use crate::util::parse_num::parse_i32;
use itertools::Itertools;
use num_traits::ToPrimitive;
use regex::Regex;
use std::collections::LinkedList;
use std::iter::successors;

type MovingRobot = (Diff, Diff);
const FILLED: [&str; 2] = ["◼️", "◻️"];

#[allow(dead_code)]
pub fn get_safety_score(lines: &[String], bounds: &Index2D) -> usize {
    let half_x = (bounds.0 / 2).to_i32().unwrap();
    let half_y = (bounds.1 / 2).to_i32().unwrap();
    let seconds = 100;
    let extractor = generate_extractor();

    let diff = bounds.as_diff();

    let after_move = &lines.iter()
        .map(|l| parse(l, &extractor))
        .map(|robot| diff.move_robot(&robot, seconds).0)
        .collect_vec();

    count(after_move, &|robot| robot.0 < half_x && robot.1 < half_y)
        * count(after_move, &|robot| robot.0 < half_x && robot.1 > half_y)
        * count(after_move, &|robot| robot.0 > half_x && robot.1 < half_y)
        * count(after_move, &|robot| robot.0 > half_x && robot.1 > half_y)
}

#[allow(dead_code)]
pub fn get_similar_to_tree(lines: &[String]) -> usize {
    let bounds = Diff(101, 103);
    let extractor = generate_extractor();

    let initial_position = lines.iter()
        .map(|l| parse(l, &extractor))
        .collect_vec();

    successors(Some(initial_position), |robots| {
        Some(
            robots.iter()
                .map(|robot| bounds.move_robot(robot, 1))
                .collect_vec()
        )
    }).enumerate().filter_map(|(second, robots)| {
        Some((second, Bounded::from(&bounds, &robots)))
            .take_if(|(_, map)| map.has_big_cluster(&robots))
    })
        .inspect(|(second, map)| map.print(*second))
        .nth(0).unwrap().0
}

fn count<T>(positions: &Vec<T>, matches: &dyn Fn(&&T) -> bool) -> usize {
    positions.iter().filter(matches).count()
}

fn parse(line: &String, extractor: &Regex) -> MovingRobot {
    let (_, [x, y, diff_x, diff_y]) = extractor.captures(line).unwrap().extract();

    (Diff(parse_i32(x), parse_i32(y)), Diff(parse_i32(diff_x), parse_i32(diff_y)))
}

impl Diff {
    fn move_robot(&self, (previous, diff): &MovingRobot, amount: usize) -> MovingRobot {
        (self.constrains_to(&((diff * amount) + previous)), *diff)
    }

    fn constrains_to(&self, Diff(diff_x, diff_y): &Diff) -> Diff {
        let x = diff_x.rem_euclid(self.0);
        let y = diff_y.rem_euclid(self.1);

        Diff(x, y)
    }
}

impl Bounded<bool> {
    fn from(bounds: &Diff, robots: &[MovingRobot]) -> Bounded<bool> {
        let mut content = vec![vec![false; bounds.0 as usize]; bounds.1 as usize];

        robots.iter()
            .for_each(|(Diff(x, y), _)| content[*y as usize][*x as usize] = true);

        Bounded {
            content,
            height: bounds.1 as usize,
            width: bounds.0 as usize,
        }
    }

    fn print(&self, second: usize) {
        println!("=======================================================================================");
        println!("====================================    {second:4}       ====================================");
        println!("=======================================================================================");

        self.print_by(&|_, filled| FILLED[*filled as usize])
    }

    fn has_big_cluster(&self, robots: &[MovingRobot]) -> bool {
        let cluster_min_size = robots.len() / 3;
        let skippable = robots.len() - (robots.len() / 4);

        let mut visitable = self.clone();

        robots.iter()
            // As we are searching for a big cluster, doing that lightens the load a little
            // with low risk, as it is highly improbable that the whole cluster not distributed
            .skip(skippable)
            .any(|(position, _)| self.capture_group_size(position, &mut visitable) > cluster_min_size)
    }

    fn capture_group_size(&self, position: &Diff, visitable: &mut Bounded<bool>) -> usize {
        let position = Index2D::from_diff(position).unwrap();

        let mut to_visit = LinkedList::from([position]);
        let mut contained = 0usize;
        visitable[&position] = false;

        while !to_visit.is_empty() {
            let curr = to_visit.pop_front().unwrap();
            contained += 1;

            let adjacent = visitable.find_adjacent_with_content(&curr)
                .filter(|(_, &available)| available)
                .map(|(next, _)| next)
                .collect_vec();

            for next in adjacent {
                visitable[&next] = false;
                to_visit.push_front(next);
            }
        }

        contained
    }
}

fn generate_extractor() -> Regex {
    Regex::new("(\\d+),(\\d+).{3}([-0-9]+),([-0-9]+)").unwrap()
}
