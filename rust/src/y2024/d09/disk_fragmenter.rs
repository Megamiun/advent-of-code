use crate::y2024::d09::helper::{get_file_space_pairs, get_files_and_blanks};
use itertools::{repeat_n, Itertools};
use std::collections::VecDeque;
use std::iter::repeat_with;

pub fn fragment(lines: &[String]) -> usize {
    let chuncked = get_file_space_pairs(lines);

    let numbers = &mut chuncked.iter().enumerate()
        .flat_map(|(index, (file_size, _))| repeat_n(index, *file_size))
        .collect::<VecDeque<_>>();

    chuncked.iter().map(|(file, blank)|
        vec![
            get_n(*file, || numbers.pop_front()),
            get_n(*blank, || numbers.pop_back())
        ]
    ).flatten().flatten().enumerate()
        .map(|(index, value)| index * value)
        .sum()
}

pub fn reorder(lines: &[String]) -> usize {
    let (files, blanks) = get_files_and_blanks(lines);

    files.iter().rev().map(|[start, file_size, index]| {
        let first_blank = blanks.iter()
            .filter(|blank| blank.borrow()[0] < *start)
            .filter(|blank| blank.borrow()[1] >= *file_size)
            .nth(0);

        match first_blank {
            Some(blank_cell) => {
                let mut mut_blank = blank_cell.borrow_mut();
                let new_start = mut_blank[0];
                mut_blank[0] += *file_size;
                mut_blank[1] -= *file_size;
                
                (new_start..new_start + *file_size).sum::<usize>() * index
            },
            _ => (*start..*start + *file_size).sum::<usize>() * index
        }
    }).sum()
}

fn get_n(amount: usize, get: impl FnMut() -> Option<usize>) -> Vec<usize> {
    repeat_with(get)
        .take_while(|num| num.is_some())
        .take(amount)
        .flatten()
        .collect_vec()
}
