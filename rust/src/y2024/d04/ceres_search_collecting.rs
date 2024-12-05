use std::convert::TryFrom;

static XMAS: &[char] = &['X', 'M', 'A', 'S'];
static SAMX: &[char] = &['S', 'A', 'M', 'X'];

pub fn find_all_xmas_collecting(lines: &Vec<String>) -> usize {
    let all_lines = [
        collect_lines(lines),
        collect_columns(lines).as_ref(),
        collect_diagonals(lines).as_ref()
    ].concat();

    all_lines
        .iter()
        .map(|t| {
            t.chars().collect::<Vec<_>>()
                .windows(4)
                .filter(|&window| window == XMAS || window == SAMX)
                .count()
        }).sum()
}

fn collect_lines(lines: &Vec<String>) -> &[String] {
    lines
}

fn collect_columns(lines: &Vec<String>) -> Box<[String]> {
    let diff = &(1, 0);

    (0..lines.get(0).unwrap().len())
        .map(|y| collect_until_end(lines, &(0, y), diff))
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn collect_diagonals(lines: &Vec<String>) -> Box<[String]> {
    let diff_right = &(1, 1);
    let diff_left = &(-1, 1);

    let height = lines.len();
    let width = lines.get(0).unwrap().len();

    let from_top_to_right = (0..height)
        .map(|y| collect_until_end(lines, &(0, y), diff_right))
        .collect::<Vec<_>>();

    let from_left = (1..width)
        .map(|x| collect_until_end(lines, &(x, 0), diff_right))
        .collect::<Vec<_>>();

    let from_top_to_left = (0..height)
        .map(|x| collect_until_end(lines, &(x, 0), diff_left))
        .collect::<Vec<_>>();

    let from_right = (1..width)
        .map(|y| collect_until_end(lines, &(width - 1, y), diff_left))
        .collect::<Vec<_>>();

    [from_top_to_right, from_left, from_top_to_left, from_right].concat().into_boxed_slice()
}

fn collect_until_end(lines: &[String], start: &(usize, usize), diff: &(i32, i32)) -> String {
    std::iter::successors(
        Option::from(*start),
        |prev| add(prev, diff)
    ).map_while(|pos| get_char_at(lines, &pos))
        .collect()
}

fn get_char_at(lines: &[String], (x, y): &(usize, usize)) -> Option<char> {
    lines.get(*x).map(|line| line.as_bytes().get(*y).map(|c| char::from(*c))).flatten()
}

fn add((x, y): &(usize, usize), (x_diff, y_diff): &(i32, i32)) -> Option<(usize, usize)> {
    let new_x = usize::try_from(*x as i32 + x_diff);
    let new_y = usize::try_from(*y as i32 + y_diff);

    match (new_x, new_y) {
        (Ok(new_x_val), Ok(new_y_val)) => Some((new_x_val, new_y_val)),
        _ => None,
    }
}
