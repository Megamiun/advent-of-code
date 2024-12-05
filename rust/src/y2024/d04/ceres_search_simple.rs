use std::convert::TryFrom;

static DIRECTIONS: &[(i32, i32)] = &[(1, 1), (0, 1), (1, 0), (0, -1)];

static MAS: &[char] = &['M', 'A', 'S'];
static XAM: &[char] = &['X', 'A', 'M'];

pub fn find_all_xmas_simple(lines: &Vec<String>) -> usize {
    let height = lines.len();
    let width = lines[0].len();

    (0..height).map(|x| {
        (0..width).map(|y| {
            let position = &(x, y);
            let char = get_char_at(lines, position).unwrap();

            match char {
                'X' => DIRECTIONS.iter().filter(|&dir| is_valid(lines, position, dir, MAS)).count(),
                'S' => DIRECTIONS.iter().filter(|&dir| is_valid(lines, position, dir, XAM)).count(),
                _ => 0
            }
        }).sum::<usize>()
    }).sum()
}

fn is_valid(lines: &[String], start: &(usize, usize), diff: &(i32, i32), word: &[char]) -> bool {
    let value = std::iter::successors(
            Option::from(add(start, diff)),
            |prev| add(prev, diff)
        ).map_while(|pos| get_char_at(lines, &pos))
        .take(word.len())
        .collect::<Vec<char>>();

    value == *word
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
