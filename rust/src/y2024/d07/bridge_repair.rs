use crate::util::parse_num::parse_usize;

const OPERATORS: &[fn(usize, usize) -> usize] = &[
    |a, b| a + b,
    |a, b| a * b
];

const OPERATORS_EXTRA: &[fn(usize, usize) -> usize] = &[
    |a, b| a + b,
    |a, b| a * b,
    |a, b| a * 10_usize.pow(b.ilog10() + 1) + b
];

#[allow(dead_code)]
pub fn with_basic_operators(lines: &[String]) -> usize {
    run_with_operators(lines, OPERATORS)
}

#[allow(dead_code)]
pub fn with_extra_operators(lines: &[String]) -> usize {
    run_with_operators(lines, OPERATORS_EXTRA)
}

fn run_with_operators(lines: &[String], ops: &[fn(usize, usize) -> usize]) -> usize {
    parse_inputs(lines)
        .filter(|(goal, numbers)| can_achieve_goal(*goal, numbers[0], &numbers[1..], ops))
        .map(|(goal, _)| goal)
        .sum()
}

fn can_achieve_goal(goal: usize, acc: usize, next: &[usize], ops: &[fn(usize, usize) -> usize]) -> bool {
    if acc > goal {
        return false
    }
    if next.len() == 0 {
        return goal == acc
    }

    ops.iter().any(|op| can_achieve_goal(goal, op(acc, next[0]), &next[1..], ops))
}

fn parse_inputs(lines: &[String]) -> impl Iterator<Item=(usize, Vec<usize>)> + '_ {
    lines.iter().map(|line| {
        let parts = line.split(": ").collect::<Vec<_>>();
        let numbers = parts[1].split(" ").map(parse_usize).collect::<Vec<_>>();

        (parse_usize(parts[0]), numbers)
    })
}
