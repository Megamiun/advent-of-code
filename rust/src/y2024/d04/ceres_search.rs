use std::convert::TryFrom;

static DIRECTIONS: & [(i32, i32)] = &[(1, 1), (0, 1), (1, 0), (0, -1)];

pub fn find_all_xmas(lines: &Vec<String>) -> usize {
    let height = lines.len();
    let width = lines.get(0).unwrap().len();

    (0..height).map(|x| {
        (0..width).map(|y| {
            let pos = (x, y);

            DIRECTIONS
                .iter()
                .filter(|&diff| is_valid(&lines, &pos, diff))
                .count()
        }).sum::<usize>()
    }).sum::<usize>()
}

fn is_valid(lines: &[String], start: &(usize, usize), diff: &(i32, i32)) -> bool {
    let seq = std::iter::successors(Option::from(*start), |prev| add(prev, diff))
        .take(4)
        .flat_map(|(x, y)| {
            lines.get(x).map(|line| line.chars().nth(y)).flatten()
        });

    let value = seq.collect::<String>();
    value == "XMAS" || value == "SAMX"
}

fn add((x, y): &(usize, usize), (xDiff, yDiff): &(i32, i32)) -> Option<(usize, usize)> {
    let new_x = usize::try_from(*x as i32 + xDiff);
    let new_y = usize::try_from(*y as i32 + yDiff);

    match (new_x, new_y) {
        (Ok(new_x_val), Ok(new_y_val)) => Some((new_x_val, new_y_val)),
        _ => None,
    }
}
