use std::collections::VecDeque;
use std::iter::repeat_with;

pub fn part1(lines: &[String]) -> usize {
    let reference = &lines[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let mut numbers = (0..reference.len())
        .filter(|index| index % 2 == 0)
        .flat_map(|index| {
            repeat_with(|| index / 2)
                .take(reference[index])
                .collect::<Vec<_>>()
        })
        .collect::<VecDeque<_>>();

    let mut total = Vec::<usize>::new();
    let mut reference_iter = reference.iter().peekable();

    loop {
        if reference_iter.peek().is_some() {
            get_from_front(&mut numbers, &mut total, *reference_iter.next().unwrap());
        }

        if reference_iter.peek().is_some() {
            get_from_back(&mut numbers, &mut total, *reference_iter.next().unwrap());
        } else {
            let remaining = numbers.len();
            get_from_back(&mut numbers, &mut total, remaining);
            
            return (0..total.len()).map(|index| index * total[index]).sum()
        }
    }
}

pub fn part2(lines: &[String]) -> usize {
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