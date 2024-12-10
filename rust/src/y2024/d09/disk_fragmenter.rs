use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter::repeat_with;

type Metadata = (usize, usize, usize, usize);

pub fn fragment(lines: &[String]) -> usize {
    let chuncked = get_file_space_pair(&lines);

    let numbers = &mut chuncked.iter().enumerate()
        .flat_map(|(index, (file_size, _))| {
            repeat_with(|| index).take(*file_size).collect::<Vec<_>>()
        }).collect::<VecDeque<_>>();

    let total = &mut Vec::<usize>::new();

    chuncked.iter().for_each(|(file, space)| {
        get_from_front(numbers, total, *file);

        if let Some(amount) = space {
            get_from_back(numbers, total, *amount);
        } else {
            let remaining = numbers.len();
            get_from_back(numbers, total, remaining);
        };
    });

    (0..total.len()).map(|index| index * total[index]).sum()
}

pub fn reorder(lines: &[String]) -> usize {
    let chuncked = get_file_space_pair(&lines);

    let (files, blanks): (Vec<Metadata>, Vec<RefCell<Metadata>>) = chuncked
        .iter()
        .enumerate()
        .scan(0usize, |blank_end, (index, (file_size, blank_size))| {
            let start = *blank_end;
            // This scan MAKES you mutate the state, instead of reusing the previous one
            let file_end = start + file_size;
            *blank_end = file_end + blank_size.unwrap_or(0);

            Some((
                (start, *file_size, file_end, index),
                RefCell::new((file_end, blank_size.unwrap_or(0), *blank_end, index))
            ))
        }).unzip();

    files.iter().rev().map(|(start, size, _, index)| {
        let first_blank = blanks.iter()
            .filter(|blank| blank.borrow().2 < *start)
            .filter(|blank| blank.borrow().1 >= *size)
            .nth(0);

        match first_blank {
            Some(blank_cell) => {
                let mut mut_blank = blank_cell.borrow_mut();

                let new_start = mut_blank.0;

                mut_blank.0 += *size;
                mut_blank.1 -= *size;

                (new_start..new_start + *size).sum::<usize>() * index
            },
            _ => (*start..*start + *size).sum::<usize>() * index
        }
    }).sum()
}

fn get_file_space_pair(lines: &[String]) -> Vec<(usize, Option<usize>)> {
    lines[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|s| (s[0], s.get(1).cloned()))
        .collect::<Vec<_>>()
}

fn get_from_back(numbers: &mut VecDeque<usize>, total: &mut Vec<usize>, amount: usize) {
    repeat_with(|| numbers.pop_back())
        .take(amount)
        .flatten()
        .for_each(|s| total.push(s));
}

fn get_from_front(numbers: &mut VecDeque<usize>, total: &mut Vec<usize>, amount: usize) {
    repeat_with(|| numbers.pop_front())
        .take(amount)
        .flatten()
        .for_each(|s| total.push(s));
}
