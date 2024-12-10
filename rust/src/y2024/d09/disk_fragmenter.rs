use std::collections::VecDeque;
use std::iter::repeat_with;

pub fn fragment(lines: &[String]) -> usize {
    let chuncked = &lines[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|s| (s[0], s.get(1).cloned()))
        .collect::<Vec<_>>();

    let mut numbers = (0..chuncked.len())
        .flat_map(|index| {
            repeat_with(|| index)
                .take(chuncked[index].0)
                .collect::<Vec<_>>()
        })
        .collect::<VecDeque<_>>();

    let mut total = Vec::<usize>::new();

    chuncked.iter().for_each(|(file, space)| {
        get_from_front(&mut numbers, &mut total, *file);

        if let Some(amount) = space {
            get_from_back(&mut numbers, &mut total, *amount);
        } else {
            let remaining = numbers.len();
            get_from_back(&mut numbers, &mut total, remaining);
        };
    });

    (0..total.len()).map(|index| index * total[index]).sum()
}

pub fn reorder(lines: &[String]) -> usize {

    0
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
