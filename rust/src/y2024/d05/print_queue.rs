pub fn get_sum_of_correct_middle_points(lines: &Vec<String>) -> usize {
    let (rules, instructions) = parse_inputs(lines);

    instructions
        .iter()
        .filter(|&instruction| satisfies_rules(&rules, instruction))
        .map(|instruction| instruction[(instruction.len() - 1) / 2])
        .sum()
}

pub fn get_sum_of_incorrect_middle_points(lines: &Vec<String>) -> usize {
    let (rules, instructions) = parse_inputs(lines);

    instructions
        .iter()
        .filter(|&instruction| !satisfies_rules(&rules, instruction))
        .map(|instruction| fix_instruction(&rules, instruction))
        .map(|instruction| instruction[(instruction.len() - 1) / 2])
        .sum()
}

fn fix_instruction(rules: &Vec<(usize, usize)>, instruction: &Vec<usize>) -> Vec<usize> {
    let mut copy = instruction.iter().copied().collect::<Vec<_>>();
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

fn satisfies_rules(rules: &Vec<(usize, usize)>, instruction: &Vec<usize>) -> bool {
    rules.iter().all(|rule| is_valid(instruction, rule))
}

fn is_valid(instruction: &Vec<usize>, rule: &(usize, usize)) -> bool {
    let positions = get_positions(instruction, rule);
    positions.is_none() || positions.is_some_and(|(f, s)| f < s)
}

fn get_positions(instruction: &Vec<usize>, (pre, pos): &(usize, usize)) -> Option<(usize, usize)> {
    let pre_position = instruction.iter().position(|v| v == pre);
    let post_position = instruction.iter().position(|v| v == pos);

    match (pre_position, post_position) {
        (Some(pre_index), Some(post_index)) => Option::from((pre_index, post_index)),
        _ => None,
    }
}

fn parse_inputs(lines: &Vec<String>) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let grouped = lines
        .split(|line| line.is_empty())
        .take(2)
        .collect::<Vec<_>>();

    let rules = grouped[0]
        .iter()
        .map(|line| line.split("|").map(&to_usize).collect::<Vec<usize>>())
        .map(|content| (content[0], content[1]))
        .collect::<Vec<_>>();

    let instructions = grouped[1]
        .iter()
        .map(|line| line.split(",").map(&to_usize).collect::<Vec<usize>>())
        .collect::<Vec<_>>();
    (rules, instructions)
}

fn to_usize(num: &str) -> usize {
    usize::from_str_radix(num, 10).unwrap()
}