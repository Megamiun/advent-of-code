use std::collections::HashMap;
use std::ops::Deref;
use std::sync::LazyLock;
use ibig::UBig;

const ZERO: LazyLock<UBig> = LazyLock::new(||UBig::from(0u32));
const ONE: LazyLock<UBig> = LazyLock::new(||UBig::from(1u32));
const TEN: LazyLock<UBig> = LazyLock::new(||UBig::from(10u32));
const OTHER_MULT: LazyLock<UBig> = LazyLock::new(||UBig::from(2024u32));

pub fn after_25_steps(lines: &[String]) -> usize {
    stones_after_steps(lines, 25)
}

pub fn after_75_steps(lines: &[String]) -> usize {
    stones_after_steps(lines, 75)
}

fn stones_after_steps(lines: &[String], steps: usize) -> usize {
    lines[0]
        .split(" ")
        .map(|number| UBig::from_str_radix(number, 10).unwrap())
        .map(|number| apply_rules(steps, &number, &mut HashMap::new()))
        .sum::<usize>()
}

fn apply_rules<'a>(times: usize, number: &UBig, cache: &mut HashMap<(UBig, usize), usize>) -> usize {
    if times == 0 {
        return 1;
    }

    if let Some(count) = cache.get(&(number.clone(), times)) {
        return *count;
    }

    let result = calculate(times, &number, cache);
    cache.insert((number.clone(), times), result);
    result
}

fn calculate(times: usize, number: &UBig, cache: &mut HashMap<(UBig, usize), usize>) -> usize {
    let next_times = times - 1;
    if number == ZERO.deref() {
        return apply_rules(next_times, ONE.deref(), cache);
    }

    let length = number.in_radix(10).to_string().len();
    if length % 2 == 0 {
        let separator = TEN.pow(length / 2);
        let first_half = number / separator.clone();
        let second_half = number - (first_half.clone() * separator.clone());
        apply_rules(next_times, &first_half, cache) + apply_rules(next_times, &second_half, cache)
    } else {
        apply_rules(next_times, &(number * OTHER_MULT.deref()), cache)
    }
}
