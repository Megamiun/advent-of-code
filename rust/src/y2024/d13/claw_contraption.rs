use crate::util::Index2D;
use regex::Regex;
use std::sync::LazyLock;
use std::usize;

const EXTRACTOR: LazyLock<Regex> = LazyLock::new(|| Regex::new("(\\d+).*?(\\d+)").unwrap());

pub fn pt1(groups: &[&[String]]) -> usize {
    groups
        .iter()
        .filter_map(|group| {
            let items = group
                .iter()
                .map(|line| {
                    let (_, [x, y]) = EXTRACTOR.captures(line).unwrap().extract();
                    Index2D(to_usize(x), to_usize(y))
                })
                .collect::<Vec<_>>();

            match items.as_slice() {
                [a, b, goal] => get_min_tokens(a, b, goal, 100),
                _ => None,
            }
        })
        .sum()
}

pub fn pt2(groups: &[&[String]]) -> usize {
    groups
        .iter()
        .filter_map(|group| {
            let items = group
                .iter()
                .map(|line| {
                    let (_, [x, y]) = EXTRACTOR.captures(line).unwrap().extract();
                    Index2D(to_usize(x), to_usize(y))
                })
                .collect::<Vec<_>>();

            match items.as_slice() {
                [a, b, goal] => get_min_tokens(a, b, &(goal + &Index2D(10000000000000, 10000000000000)), usize::MAX),
                _ => None,
            }
        })
        .sum()
}

fn get_min_tokens(
    button_a: &Index2D,
    button_b: &Index2D,
    goal: &Index2D, 
    limit: usize
) -> Option<usize> {
    let max_b = usize::min(usize::min(goal.0 / button_b.0, goal.1 / button_b.1), limit);
    
    println!("Starting to seek {goal} with {button_a} and {button_b}. Maximum B: {max_b}");

    (0..=max_b)
        .rev()
        .filter_map(|b| {
            let b_curr = Index2D(b * button_b.0, b * button_b.1);
            let remaining = (goal - b_curr).unwrap();

            let a = usize::min(remaining.0 / button_a.0, remaining.1 / button_a.1);
            let a_curr = Index2D(a * button_a.0, a * button_a.1);

            if a_curr == remaining {
                Some(b + (a * 3))
            } else {
                None
            }
        }).min()
}

// fn get_min_tokens(curr: &Index2D, button_a: &Index2D, button_b: &Index2D, goal: &Index2D, tokens: usize, limit: usize, max: usize) -> Option<usize> {
//     if tokens >= max {
//         return None
//     }
//
//     if curr == goal {
//         return Some(tokens)
//     }
//
//     if limit == 0 {
//         return None
//     }
//
//     if curr.0 > goal.0 || curr.1 > goal.0 {
//         return None
//     }
//
//     let right = get_min_tokens(&(curr + button_a), button_a, button_b, goal, tokens + 5, limit - 1, max)
//         .unwrap_or(usize::MAX);
//
//     let left = get_min_tokens(&(curr + button_a), button_a, button_b, goal, tokens + 5, limit - 1, usize::max(right, max))
//         .unwrap_or(usize::MAX);
//
//     Some(usize::max(left, right))
// }

fn to_usize(x: &str) -> usize {
    usize::from_str_radix(x, 10).unwrap()
}
