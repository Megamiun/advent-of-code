const OPERATORS: &[fn(i64, i64) -> i64] = &[
    |a, b| a + b,
    |a, b| a * b
];

const OPERATORS_EXTRA: &[fn(i64, i64) -> i64] = &[
    |a, b| a + b,
    |a, b| a * b,
    |a, b| a * 10_i64.pow(b.ilog10() + 1) + b
];

pub fn with_basic_operators(lines: &[String]) -> i64 {
    run_with_operators(lines, OPERATORS)
}

pub fn with_extra_operators(lines: &[String]) -> i64 {
    run_with_operators(lines, OPERATORS_EXTRA)
}

fn run_with_operators(lines: &[String], ops: &[fn(i64, i64) -> i64]) -> i64 {
    parse_inputs(lines)
        .iter()
        .filter(|(goal, numbers)| can_achieve_goal(*goal, numbers[0], &numbers[1..], ops))
        .map(|(goal, _)| goal)
        .sum()
}

fn can_achieve_goal(goal: i64, acc: i64, next: &[i64], ops: &[fn(i64, i64) -> i64]) -> bool {
    if acc > goal {
        return false
    }
    if next.len() == 0 {
        return goal == acc
    }

    ops.iter().any(|op| can_achieve_goal(goal, op(acc, next[0]), &next[1..], ops))
}

fn parse_inputs(lines: &[String]) -> Vec<(i64, Vec<i64>)> {
    lines.iter().map(|line| {
        let parts = line.split(": ").collect::<Vec<_>>();
        let numbers = parts[1].split(" ").map(to_i64).collect::<Vec<_>>();

        (to_i64(parts[0]), numbers)
    }).collect()
}

fn to_i64(num: &str) -> i64 {
    i64::from_str_radix(num, 10).unwrap()
}
