const OPERATORS: &[fn(i64, i64) -> i64] = &[
    |a, b| a + b,
    |a, b| a * b
];

const OPERATORS_PT2: &[fn(i64, i64) -> i64] = &[
    |a, b| a + b,
    |a, b| a * b,
    |a, b| a * 10_i64.pow(b.ilog10() + 1) + b
];

pub fn part1(lines: &[String]) -> i64 {
    parse_inputs(lines)
        .iter()
        .filter(|(goal, numbers)| can_achieve_goal(*goal, numbers, OPERATORS))
        .map(|(goal, _)| goal)
        .sum()
}

pub fn part2(lines: &[String]) -> i64 {
    parse_inputs(lines)
        .iter()
        .filter(|(goal, numbers)| can_achieve_goal(*goal, numbers, OPERATORS_PT2))
        .map(|(goal, _)| goal)
        .sum()
}

fn can_achieve_goal(goal: i64, next: &[i64], ops: &[fn(i64, i64) -> i64]) -> bool {
    can_achieve_goal_with_reduce(goal, next[0], &next[1..], ops)
}

fn can_achieve_goal_with_reduce(goal: i64, curr: i64, next: &[i64], ops: &[fn(i64, i64) -> i64]) -> bool {
    if curr > goal {
        return false
    }

    if next.len() == 0 {
        return goal == curr
    }

    ops.iter().any(|op|
        can_achieve_goal_with_reduce(goal, op(curr, next[0]), &next[1..], ops))
}

fn parse_inputs(lines: &[String]) -> Vec<(i64, Vec<i64>)> {
    lines
        .iter()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<_>>();

            let numbers = parts[1].split(" ").map(to_i64).collect::<Vec<_>>();

            (to_i64(parts[0]), numbers)
        })
        .collect()
}

fn to_i64(num: &str) -> i64 {
    i64::from_str_radix(num, 10).unwrap()
}
