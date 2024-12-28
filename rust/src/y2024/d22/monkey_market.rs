use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};
use std::iter::successors;
use std::ops::BitXor;
use itertools::Itertools;
use crate::util::parse_num::parse_usize;

#[allow(dead_code)]
pub fn get_sum_of_secrets_after(lines: &[String]) -> usize {
    lines.iter()
        .map(|line| parse_usize(line))
        .map(|seed| get_secret_numbers(seed).nth(2000).unwrap()).sum()
}

#[allow(dead_code)]
pub fn get_max_bananas_after_4_numbers(lines: &[String]) -> usize {
    let mut payout_per_seq_per_monkey = FxHashMap::with_capacity_and_hasher(3000, Default::default());
    
    lines.iter()
        .map(|line| parse_usize(line))
        .for_each(|seed| populate_payouts(seed, &mut payout_per_seq_per_monkey));
    
    *payout_per_seq_per_monkey.values().max().unwrap()
}

fn populate_payouts(seed: usize, sequence_payout: &mut FxHashMap<[i8; 4], usize>) {
    let mut visited =
        FxHashSet::<[i8; 4]>::with_capacity_and_hasher(2000, FxBuildHasher::default());

    let daily_numbers = get_secret_numbers(seed)
        .take(2000)
        .map(|secret| secret % 10)
        .collect_vec();

    let diffs = daily_numbers
        .windows(2)
        .map(|nums| (nums[1] as i8 - nums[0] as i8, nums[1]))
        .collect_vec();
    
    diffs.windows(4).for_each(|diff_seq| {
        if let [a1, a2, a3, a4] = diff_seq {
            let key = [a1.0, a2.0, a3.0, a4.0];
            if visited.insert(key) {
                let value_pointer = sequence_payout.entry(key).or_insert_with(|| 0);
                *value_pointer += a4.1
            }
        };
    });
}

fn get_secret_numbers(first_secret: usize) -> impl Iterator<Item=usize> {
    successors(Some(first_secret), |&secret| {
        let secret = prune(mix(secret, secret * 64));
        let secret = prune(mix(secret, secret / 32));
        let secret = prune(mix(secret, secret * 2048));

        Some(secret)
    }).into_iter()
}

fn mix(secret_number: usize, number: usize) -> usize {
    secret_number.bitxor(number)
}

fn prune(secret_number: usize) -> usize {
    secret_number % 16777216
}
