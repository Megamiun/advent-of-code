use crate::util::parse_num::parse_usize;
use itertools::Itertools;

#[allow(dead_code)]
pub fn get_sum_of_correct_middle_points(lines: &[&[String]; 2]) -> usize {
    let (rules, instructions) = parse_inputs(lines);

    instructions.iter()
        .filter(|&instruction| satisfies_rules(&rules, instruction))
        .map(|instruction| instruction[(instruction.len() - 1) / 2])
        .sum()
}

#[allow(dead_code)]
pub fn get_sum_of_incorrect_middle_points(lines: &[&[String]; 2]) -> usize {
    let (rules, instructions) = parse_inputs(lines);

    instructions.iter()
        .filter(|&instruction| !satisfies_rules(&rules, instruction))
        .map(|instruction| fix_instruction(&rules, instruction))
        .map(|instruction| instruction[(instruction.len() - 1) / 2])
        .sum()
}

fn fix_instruction(rules: &[(usize, usize)], instruction: &[usize]) -> Vec<usize> {
    let mut copy = instruction.to_vec();
    let mut has_changes = true;

    while has_changes {
        has_changes = false;

        for rule in rules {
            if is_valid(&copy, rule) {
                continue;
            }

            has_changes = true;

            let (l, r) = get_positions(&copy, rule).unwrap();
            copy.swap(l, r);
        }
    }

    copy
}

fn satisfies_rules(rules: &[(usize, usize)], instruction: &[usize]) -> bool {
    rules.iter().all(|rule| is_valid(instruction, rule))
}

fn is_valid(instruction: &[usize], rule: &(usize, usize)) -> bool {
    let positions = get_positions(instruction, rule);
    positions.is_none() || positions.is_some_and(|(f, s)| f < s)
}

fn get_positions(instruction: &[usize], (pre, pos): &(usize, usize)) -> Option<(usize, usize)> {
    let pre_index = instruction.iter().position(|v| v == pre)?;
    let post_index = instruction.iter().position(|v| v == pos)?;

    Some((pre_index, post_index))
}

fn parse_inputs(lines: &[&[String]; 2]) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let rules = lines[0].iter()
        .map(|line| line.split("|").map(parse_usize).collect_vec())
        .map(|content| (content[0], content[1]))
        .collect_vec();

    let instructions = lines[1].iter()
        .map(|line| line.split(",").map(parse_usize).collect_vec())
        .collect_vec();
    
    (rules, instructions)
}
