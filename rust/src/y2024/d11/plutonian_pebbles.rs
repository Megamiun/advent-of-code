use num_traits::identities::Zero;
use ibig::UBig;
use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::ops::Deref;
use std::sync::LazyLock;
use ibig::ops::DivRem;
use num_traits::One;

const TEN: LazyLock<UBig> = LazyLock::new(|| UBig::from(10u8));
const OTHER_MULT: LazyLock<UBig> = LazyLock::new(|| UBig::from(2024u32));

pub fn after_25_steps(lines: &[String]) -> usize {
    stones_after_steps(lines, 25)
}

pub fn after_75_steps(lines: &[String]) -> usize {
    stones_after_steps(lines, 75)
}

fn stones_after_steps(lines: &[String], steps: usize) -> usize {
    let mut cache = get_cache::<(UBig, usize)>();

    lines[0]
        .split(" ")
        .map(|number| UBig::from_str_radix(number, 10).unwrap())
        .map(|number| apply_rules(steps, &number, &mut cache))
        .sum::<usize>()
}

fn apply_rules(
    times: usize,
    number: &UBig,
    cache: &mut HashMap<(UBig, usize), usize, impl BuildHasher>,
) -> usize {
    if times == 0 {
        return 1;
    }

    let key = (number.clone(), times);
    if let Some(count) = cache.get(&key) {
        return *count;
    }

    let result = calculate(times, number, cache);
    cache.insert(key, result);
    result
}

fn calculate(
    times: usize,
    number: &UBig,
    cache: &mut HashMap<(UBig, usize), usize, impl BuildHasher>,
) -> usize {
    let next_times = times - 1;
    if number.is_zero() {
        return apply_rules(next_times, &UBig::one(), cache);
    }

    let length = number.to_string().len();
    if length % 2 == 0 {
        let separator = TEN.pow(length / 2);
        let (first_half, second_half) = number.div_rem(&separator);
        apply_rules(next_times, &first_half, cache) + apply_rules(next_times, &second_half, cache)
    } else {
        apply_rules(next_times, &(number * OTHER_MULT.deref()), cache)
    }
}

fn get_cache<T>() -> HashMap<T, usize, impl BuildHasher> {
    let size = 175000;
    // HashMap::with_capacity(size)
    FxHashMap::with_capacity_and_hasher(size, Default::default())
}
