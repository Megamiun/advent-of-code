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

pub fn get_calibration_value_for(words: &[String]) -> u32 {
    let points = words
        .iter()
        .map(|word| get_calibration_value(word))
        .sum::<u32>();
    points
}

fn get_calibration_value(value: &String) -> u32 {
    let mut numbers = Vec::new();

    for index in 0..value.len() {
        let character = value.chars().nth(index).unwrap();
        if character.is_numeric() {
            numbers.push(character.to_digit(10).unwrap());
            continue;
        }
        for mapping in MAPPINGS {
            if matches(&value, index, mapping) {
                numbers.push(mapping.1)
            }
        }
    }

    (numbers.first().unwrap() * 10) + numbers.last().unwrap()
}

fn matches(value: &String, index: usize, mapping: (&str, u32)) -> bool {
    let last = index + mapping.0.len();

    last < value.len() && &value[index..last] == mapping.0
}
