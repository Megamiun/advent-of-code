use std::convert::TryFrom;

static PERMUTATIONS: &[[(char, [(i32, i32); 2]); 2]] = &[
    [
        ('M', [(1, 1), (-1, 1)]),
        ('S', [(-1, -1), (1, -1)]),
    ],
    [
        ('S', [(1, 1), (1, -1)]),
        ('M', [(-1, 1), (-1, -1)]),
    ],
    [
        ('S', [(1, 1), (-1, 1)]),
        ('M', [(-1, -1), (1, -1)]),
    ],
    [
        ('M', [(1, 1), (1, -1)]),
        ('S', [(-1, 1), (-1, -1)]),
    ],
];

pub fn find_all_cross_mas(lines: &[String]) -> usize {
    let height = lines.len() - 1;
    let width = lines[0].len() - 1;

    (1..height).map(|x| {
        (1..width).filter(|&y| {
            let position = &(x, y);
            let char = get_char_at(lines, position);

            char == 'A' && is_xmas(lines, position)
        }).count()
    }).sum()
}

fn get_char_at(lines: &[String], (x, y): &(usize, usize)) -> char {
    char::from(*lines[*x].as_bytes().get(*y).unwrap())
}

fn is_xmas(lines: &[String], position: &(usize, usize)) -> bool {
    PERMUTATIONS.iter().any(|permutation|
        permutation.iter().all(|(char, diff)|
            get_char_at(lines, &add(position, &diff[0]).unwrap()) == *char &&
            get_char_at(lines, &add(position, &diff[1]).unwrap()) == *char
    ))
}

fn add((x, y): &(usize, usize), (x_diff, y_diff): &(i32, i32)) -> Option<(usize, usize)> {
    let new_x = usize::try_from(*x as i32 + x_diff);
    let new_y = usize::try_from(*y as i32 + y_diff);

    match (new_x, new_y) {
        (Ok(new_x_val), Ok(new_y_val)) => Some((new_x_val, new_y_val)),
        _ => None,
    }
}
