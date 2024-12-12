use ibig::ops::DivRem;
use ibig::UBig;
use num_traits::identities::Zero;
use num_traits::One;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::hash::{BuildHasher, RandomState};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::LazyLock;

const TEN: LazyLock<UBig> = LazyLock::new(|| UBig::from(10u8));
const OTHER_MULT: LazyLock<UBig> = LazyLock::new(|| UBig::from(2024u32));

pub fn after_25_steps(lines: &[String]) -> usize {
    stones_after_steps(lines, 25)
}

pub fn after_75_steps(lines: &[String]) -> usize {
    stones_after_steps(lines, 75)
}

fn stones_after_steps(lines: &[String], steps: usize) -> usize {
    let cache = get_solver_fx();
    // fx = ~18ms, std = ~23ms on my machine
    // let cache = get_cache();

    lines[0]
        .split(" ")
        .map(|number| Rc::new(UBig::from_str_radix(number, 10).unwrap()))
        .map(|number| cache.apply_rules(steps, &number))
        .sum::<usize>()
}

struct Solver<T: BuildHasher> {
    cache: UnsafeCell<HashMap<(UBig, usize), usize, T>>
}

impl<T: BuildHasher> Solver<T> {
    fn apply_rules(&self, times: usize, number: &UBig) -> usize {
        if times == 0 {
            return 1;
        }

        let key = (number.clone(), times);

        *self.get_cache().entry(key)
            .or_insert_with(|| self.calculate(times, number))
    }

    fn calculate(&self, times: usize, number: &UBig) -> usize {
        let next_times = times - 1;
        if number.is_zero() {
            return self.apply_rules(next_times, &UBig::one());
        }

        let length = number.to_string().len();
        if length % 2 == 0 {
            let separator = TEN.pow(length / 2);
            let (first_half, second_half) = number.div_rem(&separator);
            self.apply_rules(next_times, &first_half) + self.apply_rules(next_times, &second_half)
        } else {
            self.apply_rules(next_times, &(number * OTHER_MULT.deref()))
        }
    }

    fn get_cache(&self) -> &mut HashMap<(UBig, usize), usize, T> {
        unsafe { &mut *self.cache.get() }
    }
}

// This seems to be the capacity that gives the better performance,
// although probably overkill for the 25 steps case
const CAPACITY: usize = 65000;

#[allow(dead_code)]
fn get_solver() -> Solver<RandomState> {
    Solver { cache: UnsafeCell::new(HashMap::with_capacity(CAPACITY)) }
}

#[allow(dead_code)]
fn get_solver_fx() -> Solver<FxBuildHasher> {
    Solver { cache: UnsafeCell::new(FxHashMap::with_capacity_and_hasher(CAPACITY, Default::default())) }
}
