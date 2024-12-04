use std::convert::TryFrom;

static PERMUTATIONS: &[[(char, (i32, i32)); 4]] = &[
    [
        ('M', (1, 1)),
        ('M', (-1, 1)),
        ('S', (-1, -1)),
        ('S', (1, -1)),
    ],
    [
        ('S', (1, 1)),
        ('M', (-1, 1)),
        ('M', (-1, -1)),
        ('S', (1, -1)),
    ],
    [
        ('S', (1, 1)),
        ('S', (-1, 1)),
        ('M', (-1, -1)),
        ('M', (1, -1)),
    ],
    [
        ('M', (1, 1)),
        ('S', (-1, 1)),
        ('S', (-1, -1)),
        ('M', (1, -1)),
    ],
];

pub fn find_all_cross_mas(lines: &Vec<String>) -> usize {
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
    lines[*x].chars().nth(*y).unwrap()
}

fn is_xmas(lines: &[String], position: &(usize, usize)) -> bool {
    PERMUTATIONS.iter().any(|permutation|
        permutation.iter().all(|(char, diff)|
            get_char_at(lines, &add(position, diff).unwrap()) == *char)
    )
}

fn add((x, y): &(usize, usize), (xDiff, yDiff): &(i32, i32)) -> Option<(usize, usize)> {
    let new_x = usize::try_from(*x as i32 + xDiff);
    let new_y = usize::try_from(*y as i32 + yDiff);

    match (new_x, new_y) {
        (Ok(new_x_val), Ok(new_y_val)) => Some((new_x_val, new_y_val)),
        _ => None,
    }
}
