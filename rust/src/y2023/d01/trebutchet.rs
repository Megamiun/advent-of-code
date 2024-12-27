static MAPPINGS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

#[allow(dead_code)]
pub fn get_calibration_value_for(words: &[String]) -> u32 {
    words.iter()
        .map(|word| get_calibration_value(word, &[]))
        .sum()
}

#[allow(dead_code)]
pub fn get_calibration_value_for_written(words: &[String]) -> u32 {
    words.iter()
        .map(|word| get_calibration_value(word, &MAPPINGS))
        .sum()
}

fn get_calibration_value(value: &str, mappings: &[(&str, u32)]) -> u32 {
    let mut numbers = Vec::new();
    
    for (index, char) in value.char_indices() {
        if let Some(digit) = char.to_digit(10) {
            numbers.push(digit)
        }

        for (word, digit) in mappings {
            if matches(&value, index, word) {
                numbers.push(*digit)
            }
        }
    }

    (numbers.first().unwrap() * 10) + numbers.last().unwrap()
}

fn matches(value: &str, index: usize, mapping: &str) -> bool {
    let last = index + mapping.len();

    last <= value.len() && &value[index..last] == mapping
}
