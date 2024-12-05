use regex::{Captures, Regex};
use std::sync::LazyLock;

static MUL_FINDER: LazyLock<Regex> = LazyLock::new(|| Regex::new("mul\\((\\d+),(\\d+)\\)").unwrap());

static DO_FINDER: LazyLock<Regex> = LazyLock::new(|| Regex::new("(^|do\\(\\))(.+?)(don't\\(\\)|$)").unwrap());

pub fn sum_muls(lines: &[String]) -> u32 {
    let content = lines.iter()
        .as_slice()
        .join("\n");

    calculate_muls(&content)
}

pub fn sum_muls_in_do(lines: &[String]) -> u32 {
    let content = lines.iter()
        .as_slice()
        .join(" ");

    DO_FINDER
        .captures_iter(&content)
        .map(|do_area| calculate_muls(&do_area.get(2).unwrap().as_str().to_string()))
        .sum()
}

fn calculate_muls(line: &String) -> u32 {
    MUL_FINDER
        .captures_iter(line)
        .map(|groups| extract_u32(&groups, 1) * extract_u32(&groups, 2))
        .sum()
}

fn extract_u32(groups: &Captures, position: usize) -> u32 {
    to_u32(groups.get(position).unwrap().as_str())
}

fn to_u32(num: &str) -> u32 {
    u32::from_str_radix(num, 10).unwrap()
}
