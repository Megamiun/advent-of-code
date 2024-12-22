use rustc_hash::FxHashMap;
use std::iter::successors;
use std::ops::BitXor;
use num_traits::ToPrimitive;

pub fn get_sum_of_secrets_after(lines: &[String], rounds: usize) -> usize {
    lines.iter()
        .filter_map(|line| usize::from_str_radix(line, 10).ok())
        .map(|seed| get_secret_numbers(seed).nth(rounds - 1).unwrap()).sum()
}

pub fn get_max_bananas_after_4_numbers(lines: &[String]) -> usize {
    let mut payout_per_seq_per_monkey = FxHashMap::default();
    
    lines.iter()
        .filter_map(|line| usize::from_str_radix(line, 10).ok())
        .for_each(|seed| populate_payouts(seed, &mut payout_per_seq_per_monkey));
    
    payout_per_seq_per_monkey.values()
        .map(|payout| payout.values().sum())
        .max().unwrap()
}

fn populate_payouts(seed: usize, sequence_payout: &mut FxHashMap<[i8; 4], FxHashMap<usize, usize>>) {
    let daily_numbers = get_secret_numbers(seed)
        .take(2000)
        .map(|secret| secret % 10)
        .collect::<Vec<_>>();

    let diffs = daily_numbers
        .windows(2)
        .map(|nums| (nums[1] as i8 - nums[0] as i8, nums[1]))
        .collect::<Vec<_>>();
    
    diffs.windows(4).for_each(|diff_seq| {
        if let [a1, a2, a3, a4] = diff_seq {
            sequence_payout
                .entry([a1.0, a2.0, a3.0, a4.0]).or_insert_with(|| FxHashMap::default())
                .entry(seed).or_insert(a4.1);
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
