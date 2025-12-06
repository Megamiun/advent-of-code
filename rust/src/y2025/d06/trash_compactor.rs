use crate::util::parse_num::parse_u64;
use itertools::Itertools;
use regex::Regex;
use std::cmp::{max, min};

#[allow(dead_code)]
pub fn get_sum_of_solutions(lines: &[String]) -> u64 {
    let operators = extract_operators(lines);
    let problems = extract_problems(lines, operators.len())
        .iter().map(|nums| nums.iter().map(|num| parse_u64(num.trim())).collect_vec())
        .collect_vec();

    calculate_solution(operators, problems)
}

#[allow(dead_code)]
pub fn get_sum_of_solutions_columnar(lines: &[String]) -> u64 {
    let operators = extract_operators(lines);

    let problems = extract_problems(lines, operators.len()).iter().map(|nums| {
        let max = nums.iter().map(|num| num.len()).max().unwrap();

        (0..max)
            .map(|digit_index| extract_number_at_position(nums, digit_index))
            .collect_vec()
    }).collect_vec();

    calculate_solution(operators, problems)
}

fn calculate_solution(operators: Vec<&str>, reindexed_problems: Vec<Vec<u64>>) -> u64 {
    operators.iter().enumerate().map(|(index, &operator_content)| {
        let operator = match operator_content {
            "+" => |a, b| a + b,
            _ => |a, b| a * b
        };
        let operator_zero: u64 = match operator_content {
            "+" => 0,
            _ => 1
        };

        reindexed_problems[index].iter().fold(operator_zero, operator)
    }).sum()
}

fn extract_number_at_position(nums: &Vec<&str>, digit_index: usize) -> u64 {
    nums.iter().fold(0u64, |acc, num| {
        let digit = num.chars()
            .nth(digit_index)
            .map(|num| num.to_digit(10))
            .flatten();

        match digit {
            Some(digit) => digit as u64 + (acc * 10),
            _ => acc
        }
    })
}

fn extract_operators(lines: &[String]) -> Vec<&str> {
    lines.last()
        .map(|line| line.split_whitespace().map(|op| op).collect_vec())
        .unwrap()
}

fn extract_problems(lines: &[String], problems: usize) -> Vec<Vec<&str>> {
    let num_regex = Regex::new(r"\d+").unwrap();

    let line_problem_ranges = lines.iter().dropping_back(1)
        .map(|line| num_regex.find_iter(line).collect_vec())
        .collect_vec();

    let problem_boundaries = (0..problems).map(|problem| {
        let initial = line_problem_ranges[0][problem].range();
        line_problem_ranges.iter().fold(initial, |acc, ranges|
                min(acc.start, ranges[problem].start())..max(acc.end, ranges[problem].end()))
    }).collect_vec();

    problem_boundaries.iter().map(|boundary| {
        lines.iter().dropping_back(1).map(|line| &(line[boundary.start..min(boundary.end, line.len())])).collect_vec()
    }).collect_vec()
}
