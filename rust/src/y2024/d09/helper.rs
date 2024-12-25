use std::cell::RefCell;

pub type Metadata = [usize; 3];

pub fn get_files_and_blanks(lines: &[String]) -> (Vec<Metadata>, Vec<RefCell<Metadata>>) {
    let file_pairs = get_file_space_pairs(lines);

    file_pairs.iter().enumerate().scan(0usize, |blank_end, (index, (file_size, blank_size))| {
        let file_start = *blank_end;
        // This scan MAKES you mutate the state, instead of reusing the previous one
        let file_end = file_start + *file_size;
        *blank_end = file_end.saturating_add(*blank_size);

        Some((
            [file_start, *file_size, index],
            RefCell::new([file_end, *blank_size, index])
        ))
    }).unzip()
}

pub fn get_file_space_pairs(lines: &[String]) -> Vec<(usize, usize)> {
    lines[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|s| (s[0], *s.get(1).unwrap_or_else(|| &usize::MAX)))
        .collect()
}
