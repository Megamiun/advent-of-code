use crate::util::parse_num::parse_f64;
use derive_more::Display;
use itertools::Itertools;
use num_traits::ToPrimitive;
use regex::Regex;

type XYPair = [f64; 2];

#[allow(dead_code)]
pub fn calculate(groups: &[&[String]]) -> usize {
    sum_groups(groups, |a, b, goal|
        get_min_tokens(*a, *b, *goal, 100f64))
}

#[allow(dead_code)]
pub fn calculate_with_error(groups: &[&[String]]) -> usize {
    let error = [10000000000000f64, 10000000000000f64];

    sum_groups(groups, |a, b, goal|
        get_min_tokens(*a, *b, [goal[0] + error[0], goal[1] + error[1]], f64::MAX))
}

fn sum_groups(groups: &[&[String]], calculate: impl Fn(&XYPair, &XYPair, &XYPair) -> Option<usize>) -> usize {
    let extractor = create_extractor();

    groups.iter().map(|group| parse_group(group, &extractor))
        .filter_map(|[a, b, goal]| calculate(&a, &b, &goal))
        .sum()
}

fn get_min_tokens(
    button_a: XYPair,
    button_b: XYPair,
    goal: XYPair,
    limit: f64,
) -> Option<usize> {
    let x = &Equation { a: button_a[0], b: button_b[0], goal: goal[0] };
    let y = &Equation { a: button_a[1], b: button_b[1], goal: goal[1] };

    let b_x_div_y = x.b / y.b;

    let a_isolated = x.minus(y, &b_x_div_y);

    let a = (a_isolated.goal / a_isolated.a).round();
    let b = ((x.goal - (x.a * a)) / x.b).round();

    if a > limit || b > limit || !x.matches(a, b) || !y.matches(a, b) {
        return None
    }

    Some((a.to_usize()? * 3) + b.to_usize()?)
}

fn parse_group(group: &[String], extractor: &Regex) -> [XYPair; 3] {
    group.iter().map(|line| {
        let (_, [x, y]) = extractor.captures(line).unwrap().extract();
        [parse_f64(x), parse_f64(y)]
    }).collect_vec().try_into().unwrap()
}

#[derive(Display)]
#[display("{goal} = {a}a + {b}b")]
struct Equation {
    a: f64,
    b: f64,
    goal: f64,
}

impl Equation {
    fn minus(&self, eq: &Equation, factor: &f64) -> Equation {
        Equation {
            a: self.a - (eq.a * factor),
            b: self.b - (eq.b * factor),
            goal: self.goal - (eq.goal * factor),
        }
    }

    fn matches(&self, a: f64, b: f64) -> bool {
        self.goal == (self.a * a) + (self.b * b)
    }
}

fn create_extractor() -> Regex {
    Regex::new("(\\d+).{4}(\\d+)").unwrap()
}
