use crate::util::{Diff, Index2D};
use num_traits::ToPrimitive;
use regex::Regex;
use std::iter::successors;
use std::sync::LazyLock;
use std::thread::sleep;
use std::time::Duration;

type MovingRobot = (Index2D, Diff);

const EXTRACTOR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("(\\d+).(\\d+).{3}([-0-9]+).([-0-9]+)").unwrap());

pub fn get_safety_score(lines: &[String]) -> usize {
    let bounds = Index2D(101, 103);
    let half_x = bounds.0 / 2;
    let half_y = bounds.1 / 2;
    let seconds = 100;

    let after_move = &lines.iter()
        .map(parse)
        .map(|robot| move_robot(&robot, &bounds, seconds).0)
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

    successors(Some(initial_position), |robots| {
        Some(robots.iter()
            .map(|robot| move_robot(robot, &bounds, 1))
            .collect::<Vec<_>>())
    }).enumerate().for_each(|(second, robots)| {
        print_to_terminal(bounds, second, &robots);
    });
    0
}

fn move_robot((robot, diff): &MovingRobot, bounds: &Index2D, amount: usize) -> MovingRobot {
    (contrains_to(&((diff * amount) + robot), &bounds), *diff)
}

fn count<T>(positions: &Vec<T>, matches: &dyn Fn(&&T) -> bool) -> usize {
    positions.iter().filter(matches).count()
}

fn contrains_to(Diff(diff_x, diff_y): &Diff, Index2D(x, y): &Index2D) -> Index2D {
    let x = diff_x.rem_euclid(x.to_i32().unwrap()).to_usize().unwrap();
    let y = diff_y.rem_euclid(y.to_i32().unwrap()).to_usize().unwrap();

    Index2D(x, y)
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

fn print_to_terminal(bounds: Index2D, second: usize, robots: &Vec<(Index2D, Diff)>) {
    println!("=======================================================================================");
    println!("====================================    {second:4}       ====================================");
    println!("=======================================================================================");

    (0..bounds.1).for_each(|y| {
        (0..bounds.0).for_each(|x| {
            if robots.iter().any(|(robot, _)| robot.0 == x && robot.1 == y) {
                print!("◻️")
            } else {
                print!("◼️")
            }
        });
        println!();
    });

    sleep(Duration::from_millis(600))
}
